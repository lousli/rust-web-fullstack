use actix_web::{web, HttpResponse, Result};
use sqlx::SqlitePool;
use chrono::Utc;
use crate::models::{WeightConfig, ApiResponse};
use serde::{Deserialize, Serialize};

/// 权重配置验证请求
#[derive(Debug, Deserialize)]
pub struct WeightValidationRequest {
    pub influence_weight: f64,
    pub activity_weight: f64,
    pub quality_weight: f64,
    pub price_weight: f64,
}

/// 权重配置激活请求
// 激活权重配置请求结构（暂时保留，供未来功能扩展使用）
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ActivateConfigRequest {
    pub config_id: i64,
}

/// 获取权重配置列表
pub async fn get_weight_configs(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    let configs: Vec<WeightConfig> = sqlx::query_as(
        "SELECT id, name, description, influence_weight, activity_weight, quality_weight, price_weight, is_default, created_at, updated_at FROM weight_configs ORDER BY is_default DESC, created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(configs)))
}

/// 创建新的权重配置
pub async fn create_weight_config(
    pool: web::Data<SqlitePool>,
    config: web::Json<WeightConfig>,
) -> Result<HttpResponse> {
    let mut config = config.into_inner();
    
    // 验证权重总和是否为1.0
    let total_weight = config.influence_weight
        + config.activity_weight
        + config.quality_weight
        + config.price_weight;

    if (total_weight - 1.0).abs() > 0.01 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("权重总和必须为1.0，当前为: {:.2}", total_weight)
        )));
    }

    // 设置创建时间
    config.created_at = Some(Utc::now().naive_utc());
    config.updated_at = Some(Utc::now().naive_utc());

    // 如果设置为默认配置，先取消其他默认配置
    if config.is_default.unwrap_or(0) == 1 {
        sqlx::query("UPDATE weight_configs SET is_default = 0")
            .execute(pool.get_ref())
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    }

    let result = sqlx::query(
        r#"
        INSERT INTO weight_configs (
            name, description, influence_weight, activity_weight,
            quality_weight, price_weight, is_default, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&config.name)
    .bind(&config.description)
    .bind(config.influence_weight)
    .bind(config.activity_weight)
    .bind(config.quality_weight)
    .bind(config.price_weight)
    .bind(config.is_default.unwrap_or(0))
    .bind(config.created_at)
    .bind(config.updated_at)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            config.id = Some(result.last_insert_rowid());
            Ok(HttpResponse::Created().json(ApiResponse::success(config)))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("创建权重配置失败: {}", e)
        ))),
    }
}

/// 获取当前激活的权重配置
pub async fn get_active_weight_config(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    let config: Option<WeightConfig> = sqlx::query_as(
        "SELECT id, name, description, influence_weight, activity_weight, quality_weight, price_weight, is_default, created_at, updated_at FROM weight_configs WHERE is_default = 1 LIMIT 1"
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    match config {
        Some(config) => Ok(HttpResponse::Ok().json(ApiResponse::success(config))),
        None => {
            // 如果没有默认配置，创建并返回系统默认配置
            let default_config = WeightConfig::default();
            
            let result = sqlx::query(
                r#"
                INSERT INTO weight_configs (
                    name, description, influence_weight, activity_weight,
                    quality_weight, price_weight, is_default, created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&default_config.name)
            .bind(&default_config.description)
            .bind(default_config.influence_weight)
            .bind(default_config.activity_weight)
            .bind(default_config.quality_weight)
            .bind(default_config.price_weight)
            .bind(1)
            .bind(Utc::now().naive_utc())
            .bind(Utc::now().naive_utc())
            .execute(pool.get_ref())
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            let mut config = default_config;
            config.id = Some(result.last_insert_rowid());
            config.is_default = Some(1);
            config.created_at = Some(Utc::now().naive_utc());
            config.updated_at = Some(Utc::now().naive_utc());

            Ok(HttpResponse::Ok().json(ApiResponse::success(config)))
        }
    }
}

/// 激活指定的权重配置
pub async fn activate_weight_config(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let config_id = path.into_inner();

    // 检查配置是否存在
    let config: Option<WeightConfig> = sqlx::query_as(
        "SELECT id, name, description, influence_weight, activity_weight, quality_weight, price_weight, is_default, created_at, updated_at FROM weight_configs WHERE id = ?"
    )
    .bind(config_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if config.is_none() {
        return Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "权重配置不存在".to_string()
        )));
    }

    // 开始事务
    let mut tx = pool.begin()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 取消所有默认配置
    sqlx::query("UPDATE weight_configs SET is_default = 0")
        .execute(&mut *tx)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 设置新的默认配置
    sqlx::query("UPDATE weight_configs SET is_default = 1, updated_at = ? WHERE id = ?")
        .bind(Utc::now().naive_utc())
        .bind(config_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 提交事务
    tx.commit()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(())))
}

/// 验证权重配置
pub async fn validate_weight_config(
    request: web::Json<WeightValidationRequest>,
) -> Result<HttpResponse> {
    let weights = request.into_inner();

    // 计算权重总和
    let total_weight = weights.influence_weight
        + weights.activity_weight
        + weights.quality_weight
        + weights.price_weight;

    // 验证每个权重是否在合理范围内
    let weights_vec = vec![
        ("影响力权重", weights.influence_weight),
        ("活跃度权重", weights.activity_weight),
        ("质量权重", weights.quality_weight),
        ("价格权重", weights.price_weight),
    ];

    let mut errors = Vec::new();

    for (name, weight) in weights_vec {
        if weight < 0.0 || weight > 1.0 {
            errors.push(format!("{} 必须在0-1之间", name));
        }
    }

    // 验证总和是否为1.0
    if (total_weight - 1.0).abs() > 0.01 {
        errors.push(format!("权重总和必须为1.0，当前为: {:.2}", total_weight));
    }

    let response = if errors.is_empty() {
        serde_json::json!({
            "valid": true,
            "message": "权重配置验证通过",
            "total_weight": total_weight
        })
    } else {
        serde_json::json!({
            "valid": false,
            "message": "权重配置验证失败",
            "errors": errors,
            "total_weight": total_weight
        })
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// 医疗专用权重配置请求
#[derive(Debug, Deserialize, Serialize)]
pub struct MedicalWeightRequest {
    pub name: String,
    pub description: Option<String>,
    // 四大核心评价指标权重
    pub influence_weight: f64,      // 影响力权重
    pub activity_weight: f64,       // 活跃度权重
    pub quality_weight: f64,        // 质量权重
    pub price_weight: f64,          // 价格权重
}

/// 权重策略类型
#[derive(Debug, Deserialize, Serialize)]
pub enum WeightStrategy {
    Conservative,   // 保守型：重视价格和质量
    Aggressive,     // 积极型：重视影响力和活跃度
    Balanced,       // 平衡型：各指标权重相对均衡
    QualityFocused, // 质量型：重视内容质量
}

/// 获取医疗专用权重预设方案
pub async fn get_medical_weight_presets() -> Result<HttpResponse> {
    let presets = vec![
        serde_json::json!({
            "id": "conservative",
            "name": "保守型投放",
            "description": "适合新合作医生，重视价格和质量",
            "strategy": "Conservative",
            "weights": {
                "influence_weight": 0.2,
                "activity_weight": 0.2,
                "quality_weight": 0.3,
                "price_weight": 0.3
            },
            "suitable_scenarios": ["新医生合作", "预算有限", "风险控制"]
        }),
        serde_json::json!({
            "id": "aggressive", 
            "name": "积极型投放",
            "description": "适合已验证医生，重视影响力和活跃度",
            "strategy": "Aggressive",
            "weights": {
                "influence_weight": 0.35,
                "activity_weight": 0.35,
                "quality_weight": 0.2,
                "price_weight": 0.1
            },
            "suitable_scenarios": ["效果导向", "头部医生", "快速扩张"]
        }),
        serde_json::json!({
            "id": "balanced",
            "name": "平衡型投放", 
            "description": "标准医疗健康领域权重配置，各指标权重均衡",
            "strategy": "Balanced",
            "weights": {
                "influence_weight": 0.25,
                "activity_weight": 0.25,
                "quality_weight": 0.25,
                "price_weight": 0.25
            },
            "suitable_scenarios": ["通用场景", "综合考量", "长期合作"]
        }),
        serde_json::json!({
            "id": "quality_focused",
            "name": "质量型投放",
            "description": "适合品牌宣传，重视内容质量",
            "strategy": "QualityFocused", 
            "weights": {
                "influence_weight": 0.2,
                "activity_weight": 0.15,
                "quality_weight": 0.45,
                "price_weight": 0.2
            },
            "suitable_scenarios": ["品牌建设", "权威背书", "专业形象"]
        })
    ];

    Ok(HttpResponse::Ok().json(ApiResponse::success(presets)))
}

/// 创建医疗专用权重配置
pub async fn create_medical_weight_config(
    pool: web::Data<SqlitePool>,
    request: web::Json<MedicalWeightRequest>,
) -> Result<HttpResponse> {
    let req = request.into_inner();
    
    // 验证医疗权重总和是否为1.0
    let total_weight = req.influence_weight
        + req.activity_weight
        + req.quality_weight
        + req.price_weight;

    if (total_weight - 1.0).abs() > 0.01 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("医疗权重总和必须为1.0，当前为: {:.2}", total_weight)
        )));
    }

    // 验证权重范围
    let weights = vec![
        ("影响力权重", req.influence_weight),
        ("活跃度权重", req.activity_weight),
        ("质量权重", req.quality_weight),
        ("价格权重", req.price_weight),
    ];

    for (name, weight) in &weights {
        if *weight < 0.0 || *weight > 1.0 {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("{} 必须在0-1之间，当前值: {:.2}", name, weight)
            )));
        }
    }

    // 创建权重配置
    let mut config = WeightConfig {
        id: None,
        name: req.name,
        description: Some(req.description.unwrap_or_default()),
        influence_weight: req.influence_weight,
        activity_weight: req.activity_weight,
        quality_weight: req.quality_weight,
        price_weight: req.price_weight,
        is_default: Some(0),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let result = sqlx::query(
        r#"
        INSERT INTO weight_configs (
            name, description, influence_weight, activity_weight,
            quality_weight, price_weight, is_default, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&config.name)
    .bind(&config.description)
    .bind(config.influence_weight)
    .bind(config.activity_weight)
    .bind(config.quality_weight)
    .bind(config.price_weight)
    .bind(0)
    .bind(config.created_at)
    .bind(config.updated_at)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            config.id = Some(result.last_insert_rowid());
            Ok(HttpResponse::Created().json(ApiResponse::success(config)))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("创建医疗权重配置失败: {}", e)
        ))),
    }
}

/// 分析权重影响
pub async fn analyze_weight_impact(
    _pool: web::Data<SqlitePool>,
    request: web::Json<WeightValidationRequest>,
) -> Result<HttpResponse> {
    let weights = request.into_inner();

    // 这里可以根据历史数据分析权重配置的影响
    // 目前返回一个模拟的分析结果
    let analysis = serde_json::json!({
        "impact_analysis": {
            "influence_impact": format!("影响力权重 {:.1}% 将偏向选择大V医生", weights.influence_weight * 100.0),
            "activity_impact": format!("活跃度权重 {:.1}% 将偏向选择活跃医生", weights.activity_weight * 100.0),
            "quality_impact": format!("质量权重 {:.1}% 将偏向选择高质量内容医生", weights.quality_weight * 100.0),
            "price_impact": format!("价格权重 {:.1}% 将偏向选择性价比高的医生", weights.price_weight * 100.0)
        },
        "recommendations": [
            "建议根据投放目标调整权重配置",
            "新合作建议提高价格权重",
            "品牌推广建议提高质量权重",
            "效果投放建议提高影响力权重"
        ]
    });

    Ok(HttpResponse::Ok().json(ApiResponse::success(analysis)))
}