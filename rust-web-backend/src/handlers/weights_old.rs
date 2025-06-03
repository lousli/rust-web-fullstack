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
#[derive(Debug, Deserialize)]
pub struct ActivateConfigRequest {
    pub config_id: i32,
}

/// 获取权重配置列表
pub async fn get_weight_configs(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    let configs: Vec<WeightConfig> = sqlx::query_as(
        "SELECT * FROM weight_configs ORDER BY is_default DESC, created_at DESC"
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
      // 验证权重总和是否为100
    let total_weight = config.influence_weight
        + config.activity_weight
        + config.quality_weight
        + config.price_weight;

    if (total_weight - 100.0).abs() > 0.01 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("权重总和必须为100，当前为: {:.2}", total_weight)
        )));
    }    // 设置创建时间
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
            name, description, account_type_weight, cost_effectiveness_weight,
            data_trend_weight, performance_weight, affinity_weight, editing_weight,
            video_quality_weight, is_default, created_by, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&config.name)
    .bind(&config.description)
    .bind(config.account_type_weight)
    .bind(config.cost_effectiveness_weight)
    .bind(config.data_trend_weight)
    .bind(config.performance_weight)
    .bind(config.affinity_weight)
    .bind(config.editing_weight)
    .bind(config.video_quality_weight)
    .bind(config.is_default)
    .bind(&config.created_by)
    .bind(config.created_at)
    .bind(config.updated_at)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            config.id = Some(result.last_insert_rowid() as i32);
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
        "SELECT * FROM weight_configs WHERE is_default = true LIMIT 1"
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
                    name, description, account_type_weight, cost_effectiveness_weight,
                    data_trend_weight, performance_weight, affinity_weight, editing_weight,
                    video_quality_weight, is_default, created_by, created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&default_config.name)
            .bind(&default_config.description)
            .bind(default_config.account_type_weight)
            .bind(default_config.cost_effectiveness_weight)
            .bind(default_config.data_trend_weight)
            .bind(default_config.performance_weight)
            .bind(default_config.affinity_weight)
            .bind(default_config.editing_weight)
            .bind(default_config.video_quality_weight)
            .bind(default_config.is_default)
            .bind(&default_config.created_by)
            .bind(Utc::now())
            .bind(Utc::now())
            .execute(pool.get_ref())
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            let mut config = default_config;
            config.id = Some(result.last_insert_rowid() as i32);
            config.created_at = Some(Utc::now());
            config.updated_at = Some(Utc::now());

            Ok(HttpResponse::Ok().json(ApiResponse::success(config)))
        }
    }
}

/// 激活指定的权重配置
pub async fn activate_weight_config(
    pool: web::Data<SqlitePool>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let config_id = path.into_inner();

    // 检查配置是否存在
    let config: Option<WeightConfig> = sqlx::query_as(
        "SELECT * FROM weight_configs WHERE id = ?"
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
    sqlx::query("UPDATE weight_configs SET is_default = false")
        .execute(&mut *tx)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 设置新的默认配置
    sqlx::query("UPDATE weight_configs SET is_default = true, updated_at = ? WHERE id = ?")
        .bind(Utc::now())
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

    // 计算权重总和    let total_weight = weights.influence_weight
        + weights.activity_weight
        + weights.quality_weight
        + weights.price_weight;

    // 验证每个权重是否在合理范围内
    let weights_vec = vec![
        ("账号性质权重", weights.account_type_weight),
        ("性价比权重", weights.cost_effectiveness_weight),
        ("数据趋势权重", weights.data_trend_weight),
        ("表现力权重", weights.performance_weight),
        ("亲和力权重", weights.affinity_weight),
        ("剪辑水平权重", weights.editing_weight),
        ("视频质量权重", weights.video_quality_weight),
    ];

    let mut errors = Vec::new();

    for (name, weight) in weights_vec {
        if weight < 0.0 || weight > 100.0 {
            errors.push(format!("{} 必须在0-100之间", name));
        }
    }

    // 验证总和是否为100
    if (total_weight - 100.0).abs() > 0.01 {
        errors.push(format!("权重总和必须为100，当前为: {:.2}", total_weight));
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
    // 五大核心评价指标权重
    pub account_influence_weight: f32,      // 账号影响力权重 (22%)
    pub cost_effectiveness_weight: f32,     // 性价比权重 (35%)
    pub content_quality_weight: f32,        // 内容质量权重 (28%)
    pub medical_credibility_weight: f32,    // 医疗可信度权重 (10%)
    pub roi_prediction_weight: f32,         // ROI预测权重 (5%)
}

/// 权重策略类型
#[derive(Debug, Deserialize, Serialize)]
pub enum WeightStrategy {
    Conservative,   // 保守型：重视性价比和专业可信度
    Aggressive,     // 积极型：重视影响力和ROI预测
    Balanced,       // 平衡型：各指标权重相对均衡
    BrandFocused,   // 品牌型：重视内容质量和可信度
}

/// 获取医疗专用权重预设方案
pub async fn get_medical_weight_presets() -> Result<HttpResponse> {
    let presets = vec![
        serde_json::json!({
            "id": "conservative",
            "name": "保守型投放",
            "description": "适合新合作医生，重视性价比和专业可信度",
            "strategy": "Conservative",
            "weights": {
                "account_influence_weight": 20.0,
                "cost_effectiveness_weight": 40.0,
                "content_quality_weight": 25.0,
                "medical_credibility_weight": 12.0,
                "roi_prediction_weight": 3.0
            },
            "suitable_scenarios": ["新医生合作", "预算有限", "风险控制"]
        }),
        serde_json::json!({
            "id": "aggressive", 
            "name": "积极型投放",
            "description": "适合已验证医生，重视影响力和ROI预测",
            "strategy": "Aggressive",
            "weights": {
                "account_influence_weight": 30.0,
                "cost_effectiveness_weight": 25.0,
                "content_quality_weight": 25.0,
                "medical_credibility_weight": 8.0,
                "roi_prediction_weight": 12.0
            },
            "suitable_scenarios": ["效果导向", "头部医生", "快速扩张"]
        }),
        serde_json::json!({
            "id": "balanced",
            "name": "平衡型投放", 
            "description": "标准医疗健康领域权重配置，各指标权重均衡",
            "strategy": "Balanced",
            "weights": {
                "account_influence_weight": 22.0,
                "cost_effectiveness_weight": 35.0,
                "content_quality_weight": 28.0,
                "medical_credibility_weight": 10.0,
                "roi_prediction_weight": 5.0
            },
            "suitable_scenarios": ["通用场景", "综合考量", "长期合作"]
        }),
        serde_json::json!({
            "id": "brand_focused",
            "name": "品牌型投放",
            "description": "适合品牌宣传，重视内容质量和医疗可信度",
            "strategy": "BrandFocused", 
            "weights": {
                "account_influence_weight": 18.0,
                "cost_effectiveness_weight": 20.0,
                "content_quality_weight": 35.0,
                "medical_credibility_weight": 22.0,
                "roi_prediction_weight": 5.0
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
    
    // 验证医疗权重总和是否为100
    let total_weight = req.account_influence_weight
        + req.cost_effectiveness_weight
        + req.content_quality_weight
        + req.medical_credibility_weight
        + req.roi_prediction_weight;

    if (total_weight - 100.0).abs() > 0.01 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("医疗权重总和必须为100，当前为: {:.2}", total_weight)
        )));
    }

    // 验证权重范围
    let weights = vec![
        ("账号影响力权重", req.account_influence_weight),
        ("性价比权重", req.cost_effectiveness_weight),
        ("内容质量权重", req.content_quality_weight),
        ("医疗可信度权重", req.medical_credibility_weight),
        ("ROI预测权重", req.roi_prediction_weight),
    ];

    for (name, weight) in &weights {
        if *weight < 0.0 || *weight > 100.0 {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("{} 必须在0-100之间，当前值: {:.2}", name, weight)
            )));
        }
    }    // 转换为通用WeightConfig格式存储
    let mut config = WeightConfig {
        id: None,
        name: req.name,
        description: Some(req.description.unwrap_or_default()),
        account_type_weight: req.account_influence_weight,
        cost_effectiveness_weight: req.cost_effectiveness_weight,
        data_trend_weight: 0.0, // 医疗模式下不使用数据趋势
        performance_weight: req.content_quality_weight * 0.4, // 内容质量细分：表现力40%
        affinity_weight: req.content_quality_weight * 0.3,    // 亲和力30%
        editing_weight: req.content_quality_weight * 0.2,     // 剪辑20%
        video_quality_weight: req.content_quality_weight * 0.1, // 画面质量10%
        is_default: false,
        created_by: Some("system".to_string()),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    // 如果设置为默认配置，先取消其他默认配置
    if config.is_default {
        sqlx::query("UPDATE weight_configs SET is_default = false")
            .execute(pool.get_ref())
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    }

    let result = sqlx::query(
        r#"
        INSERT INTO weight_configs (
            name, description, account_type_weight, cost_effectiveness_weight,
            data_trend_weight, performance_weight, affinity_weight, editing_weight,
            video_quality_weight, is_default, created_by, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&config.name)
    .bind(&config.description)
    .bind(config.account_type_weight)
    .bind(config.cost_effectiveness_weight)
    .bind(config.data_trend_weight)
    .bind(config.performance_weight)
    .bind(config.affinity_weight)
    .bind(config.editing_weight)
    .bind(config.video_quality_weight)
    .bind(config.is_default)
    .bind(&config.created_by)
    .bind(config.created_at)
    .bind(config.updated_at)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            config.id = Some(result.last_insert_rowid() as i32);
            
            // 返回医疗权重格式的响应
            let response = serde_json::json!({
                "id": config.id,
                "name": config.name,
                "description": config.description,
                "account_influence_weight": req.account_influence_weight,
                "cost_effectiveness_weight": req.cost_effectiveness_weight,
                "content_quality_weight": req.content_quality_weight,
                "medical_credibility_weight": req.medical_credibility_weight,
                "roi_prediction_weight": req.roi_prediction_weight,
                "created_at": config.created_at,
                "updated_at": config.updated_at
            });
            
            Ok(HttpResponse::Created().json(ApiResponse::success(response)))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("创建医疗权重配置失败: {}", e)
        ))),
    }
}

/// 权重影响分析
pub async fn analyze_weight_impact(
    _pool: web::Data<SqlitePool>,
    request: web::Json<MedicalWeightRequest>,
) -> Result<HttpResponse> {
    let weights = request.into_inner();
    
    // 模拟分析不同权重配置对样本医生的评分影响
    // 这里使用刘翔医生的数据作为示例
    let sample_doctor = serde_json::json!({
        "name": "刘翔医生",
        "current_scores": {
            "account_influence": 85.2,
            "cost_effectiveness": 75.8,
            "content_quality": 78.5,
            "medical_credibility": 82.0,
            "roi_prediction": 73.5
        }
    });

    // 计算当前配置下的综合评分
    let current_score = 
        sample_doctor["current_scores"]["account_influence"].as_f64().unwrap_or(0.0) * weights.account_influence_weight as f64 / 100.0 +
        sample_doctor["current_scores"]["cost_effectiveness"].as_f64().unwrap_or(0.0) * weights.cost_effectiveness_weight as f64 / 100.0 +
        sample_doctor["current_scores"]["content_quality"].as_f64().unwrap_or(0.0) * weights.content_quality_weight as f64 / 100.0 +
        sample_doctor["current_scores"]["medical_credibility"].as_f64().unwrap_or(0.0) * weights.medical_credibility_weight as f64 / 100.0 +
        sample_doctor["current_scores"]["roi_prediction"].as_f64().unwrap_or(0.0) * weights.roi_prediction_weight as f64 / 100.0;

    // 分析权重分布特征
    let max_weight = weights.account_influence_weight
        .max(weights.cost_effectiveness_weight)
        .max(weights.content_quality_weight)
        .max(weights.medical_credibility_weight)
        .max(weights.roi_prediction_weight);

    let strategy_analysis = if weights.cost_effectiveness_weight >= max_weight {
        "当前配置偏向成本效益优先，适合预算控制严格的投放场景"
    } else if weights.account_influence_weight >= max_weight {
        "当前配置偏向影响力优先，适合扩大品牌曝光的投放场景"
    } else if weights.content_quality_weight >= max_weight {
        "当前配置偏向内容质量优先，适合注重专业形象的投放场景"
    } else if weights.medical_credibility_weight >= max_weight {
        "当前配置偏向专业可信度优先，适合医疗权威背书的投放场景"
    } else {
        "当前配置偏向ROI预测优先，适合效果导向的投放场景"
    };

    let analysis_result = serde_json::json!({
        "sample_analysis": {
            "doctor": sample_doctor,
            "predicted_score": format!("{:.1}", current_score),
            "score_breakdown": {
                "account_influence_contribution": format!("{:.2}", sample_doctor["current_scores"]["account_influence"].as_f64().unwrap_or(0.0) * weights.account_influence_weight as f64 / 100.0),
                "cost_effectiveness_contribution": format!("{:.2}", sample_doctor["current_scores"]["cost_effectiveness"].as_f64().unwrap_or(0.0) * weights.cost_effectiveness_weight as f64 / 100.0),
                "content_quality_contribution": format!("{:.2}", sample_doctor["current_scores"]["content_quality"].as_f64().unwrap_or(0.0) * weights.content_quality_weight as f64 / 100.0),
                "medical_credibility_contribution": format!("{:.2}", sample_doctor["current_scores"]["medical_credibility"].as_f64().unwrap_or(0.0) * weights.medical_credibility_weight as f64 / 100.0),
                "roi_prediction_contribution": format!("{:.2}", sample_doctor["current_scores"]["roi_prediction"].as_f64().unwrap_or(0.0) * weights.roi_prediction_weight as f64 / 100.0)
            }
        },
        "strategy_analysis": strategy_analysis,
        "weight_distribution": {
            "dominant_factor": if weights.cost_effectiveness_weight >= max_weight { "cost_effectiveness" } 
                             else if weights.account_influence_weight >= max_weight { "account_influence" }
                             else if weights.content_quality_weight >= max_weight { "content_quality" }
                             else if weights.medical_credibility_weight >= max_weight { "medical_credibility" }
                             else { "roi_prediction" },
            "balance_score": 100.0 - (max_weight - 20.0).max(0.0) * 2.0, // 权重越均衡分数越高
            "risk_level": if max_weight > 50.0 { "高风险：权重过于集中" } 
                         else if max_weight > 40.0 { "中风险：权重较为集中" }
                         else { "低风险：权重分布合理" }
        },
        "recommendations": generate_weight_recommendations(&weights)
    });

    Ok(HttpResponse::Ok().json(ApiResponse::success(analysis_result)))
}

/// 生成权重优化建议
fn generate_weight_recommendations(weights: &MedicalWeightRequest) -> Vec<String> {
    let mut recommendations = Vec::new();

    // 检查权重是否过于极端
    let max_weight = weights.account_influence_weight
        .max(weights.cost_effectiveness_weight)
        .max(weights.content_quality_weight)
        .max(weights.medical_credibility_weight)
        .max(weights.roi_prediction_weight);

    if max_weight > 50.0 {
        recommendations.push("建议避免单一指标权重超过50%，以保持评价的全面性".to_string());
    }

    // 医疗健康领域特定建议
    if weights.medical_credibility_weight < 5.0 {
        recommendations.push("医疗可信度权重建议不低于5%，以确保专业性评估".to_string());
    }

    if weights.content_quality_weight < 20.0 {
        recommendations.push("内容质量权重建议不低于20%，医疗内容的专业性很重要".to_string());
    }

    if weights.cost_effectiveness_weight > 45.0 {
        recommendations.push("性价比权重过高可能忽略质量因素，建议适当降低".to_string());
    }

    // 权重平衡性建议
    let weights_vec = vec![
        weights.account_influence_weight,
        weights.cost_effectiveness_weight,
        weights.content_quality_weight,
        weights.medical_credibility_weight,
        weights.roi_prediction_weight,
    ];
    
    let avg_weight = weights_vec.iter().sum::<f32>() / weights_vec.len() as f32;
    let variance = weights_vec.iter()
        .map(|w| (w - avg_weight).powi(2))
        .sum::<f32>() / weights_vec.len() as f32;

    if variance > 200.0 {
        recommendations.push("权重分布差异较大，建议考虑适当平衡各指标权重".to_string());
    }

    if recommendations.is_empty() {
        recommendations.push("当前权重配置合理，符合医疗健康领域的评价标准".to_string());
    }

    recommendations
}