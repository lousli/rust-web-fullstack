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

    if (total_weight - 1.0).abs() > 0.01 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("权重总和必须为1，当前为: {:.2}", total_weight)
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
        Err(e) => {
            if e.to_string().contains("UNIQUE") {
                Ok(HttpResponse::Conflict().json(ApiResponse::<()>::error(
                    "权重配置名称已存在".to_string()
                )))
            } else {
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }
}

/// 权重配置验证
pub async fn validate_weights(
    weights: web::Json<WeightValidationRequest>,
) -> Result<HttpResponse> {
    let total_weight = weights.influence_weight
        + weights.activity_weight
        + weights.quality_weight
        + weights.price_weight;

    if (total_weight - 1.0).abs() > 0.01 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("权重总和必须为1，当前为: {:.2}", total_weight)
        )));
    }

    let weight_details = vec![
        ("影响力权重", weights.influence_weight),
        ("活跃度权重", weights.activity_weight),
        ("质量权重", weights.quality_weight),
        ("价格权重", weights.price_weight),
    ];

    for (name, weight) in weight_details {
        if weight < 0.0 || weight > 1.0 {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("{} 必须在0-1之间，当前值: {:.2}", name, weight)
            )));
        }
    }

    #[derive(Serialize)]
    struct ValidationResult {
        valid: bool,
        total_weight: f64,
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(ValidationResult {
        valid: true,
        total_weight,
    })))
}

/// 激活权重配置
pub async fn activate_config(
    pool: web::Data<SqlitePool>,
    req: web::Json<ActivateConfigRequest>,
) -> Result<HttpResponse> {
    // 先取消所有默认配置
    sqlx::query("UPDATE weight_configs SET is_default = 0")
        .execute(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 激活指定配置
    let result = sqlx::query("UPDATE weight_configs SET is_default = 1 WHERE id = ?")
        .bind(req.config_id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if result.rows_affected() == 0 {
        return Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "权重配置不存在".to_string()
        )));
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success("配置激活成功")))
}

/// 获取默认权重配置
pub async fn get_default_config(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    let config: Option<WeightConfig> = sqlx::query_as(
        "SELECT * FROM weight_configs WHERE is_default = 1 LIMIT 1"
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    match config {
        Some(config) => Ok(HttpResponse::Ok().json(ApiResponse::success(config))),
        None => {
            // 如果没有默认配置，创建一个
            let default_config = WeightConfig::default();
            Ok(HttpResponse::Ok().json(ApiResponse::success(default_config)))
        }
    }
}

/// 删除权重配置
pub async fn delete_weight_config(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let config_id = path.into_inner();

    // 检查是否为默认配置
    let is_default: (i64,) = sqlx::query_as(
        "SELECT is_default FROM weight_configs WHERE id = ?"
    )
    .bind(config_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
    .unwrap_or((0,));

    if is_default.0 == 1 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "不能删除默认权重配置".to_string()
        )));
    }

    let result = sqlx::query("DELETE FROM weight_configs WHERE id = ?")
        .bind(config_id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if result.rows_affected() == 0 {
        return Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "权重配置不存在".to_string()
        )));
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success("删除成功")))
}
