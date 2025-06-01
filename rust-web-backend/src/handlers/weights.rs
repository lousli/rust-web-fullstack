use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use crate::models::{WeightConfig, ApiResponse};

/// 模拟权重配置数据
fn get_mock_weight_configs() -> Vec<WeightConfig> {
    vec![
        WeightConfig {
            id: 1,
            config_name: "默认均衡配置".to_string(),
            cost_performance_weight: 16.67,
            data_index_weight: 16.67,
            performance_weight: 16.67,
            affinity_weight: 16.67,
            editing_weight: 16.66,
            video_quality_weight: 16.66,
            is_active: true,
            created_at: Utc::now(),
        },
        WeightConfig {
            id: 2,
            config_name: "数据导向配置".to_string(),
            cost_performance_weight: 30.0,
            data_index_weight: 30.0,
            performance_weight: 10.0,
            affinity_weight: 10.0,
            editing_weight: 10.0,
            video_quality_weight: 10.0,
            is_active: false,
            created_at: Utc::now(),
        },
        WeightConfig {
            id: 3,
            config_name: "表现力导向配置".to_string(),
            cost_performance_weight: 10.0,
            data_index_weight: 10.0,
            performance_weight: 30.0,
            affinity_weight: 30.0,
            editing_weight: 10.0,
            video_quality_weight: 10.0,
            is_active: false,
            created_at: Utc::now(),
        },
    ]
}

/// 获取权重配置列表
pub async fn get_weight_configs() -> Result<HttpResponse> {
    let configs = get_mock_weight_configs();
    Ok(HttpResponse::Ok().json(ApiResponse::success(configs)))
}

/// 获取当前激活的权重配置
pub async fn get_active_weight_config() -> Result<HttpResponse> {
    let configs = get_mock_weight_configs();
    
    if let Some(active_config) = configs.into_iter().find(|c| c.is_active) {
        Ok(HttpResponse::Ok().json(ApiResponse::success(active_config)))
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("未找到激活的权重配置".to_string())))
    }
}

/// 创建权重配置
pub async fn create_weight_config(config: web::Json<WeightConfig>) -> Result<HttpResponse> {
    // 验证权重总和是否为100
    let total_weight = config.cost_performance_weight + 
                      config.data_index_weight + 
                      config.performance_weight + 
                      config.affinity_weight + 
                      config.editing_weight + 
                      config.video_quality_weight;
    
    if (total_weight - 100.0).abs() > 0.01 {
        return Ok(HttpResponse::BadRequest().json(
            ApiResponse::<()>::error(format!("权重总和必须为100%，当前为{:.2}%", total_weight))
        ));
    }
    
    println!("Creating weight config: {:?}", config);
    Ok(HttpResponse::Created().json(ApiResponse::success(config.into_inner())))
}

/// 激活权重配置
pub async fn activate_weight_config(path: web::Path<u32>) -> Result<HttpResponse> {
    let config_id = path.into_inner();
    println!("Activating weight config: {}", config_id);
    
    let message = format!("权重配置 {} 已激活", config_id);
    Ok(HttpResponse::Ok().json(ApiResponse::success(message)))
}

/// 验证权重配置
pub async fn validate_weight_config(config: web::Json<WeightConfig>) -> Result<HttpResponse> {
    let total_weight = config.cost_performance_weight + 
                      config.data_index_weight + 
                      config.performance_weight + 
                      config.affinity_weight + 
                      config.editing_weight + 
                      config.video_quality_weight;
    
    let is_valid = (total_weight - 100.0).abs() <= 0.01;
    
    let result = serde_json::json!({
        "is_valid": is_valid,
        "total_weight": total_weight,
        "message": if is_valid { "权重配置有效" } else { "权重总和必须为100%" }
    });
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}
