use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 医生基本信息模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Doctor {
    pub id: u32,
    pub doctor_id: String,
    pub name: String,
    pub title: String,
    pub region: String,
    pub department: String,
    pub institution: String,
    pub account_type: String, // 头部、腰部、尾部
    pub created_at: DateTime<Utc>,
}

/// 医生评分模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoctorScore {
    pub id: u32,
    pub doctor_id: u32,
    pub doctor_name: String,
    pub department: String,
    pub region: String,
    pub institution: String,
    pub account_type: String,
    pub cost_performance_score: f64,    // 性价比指数
    pub data_index_score: f64,          // 数据指数
    pub performance_score: f64,         // 医生表现力评分
    pub affinity_score: f64,            // 医生亲和力评分
    pub editing_score: f64,             // 剪辑水平评分
    pub video_quality_score: f64,       // 画面质量评分
    pub weighted_total_score: f64,      // 加权总分
    pub calculated_at: DateTime<Utc>,
}

/// 权重配置模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeightConfig {
    pub id: u32,
    pub config_name: String,
    pub cost_performance_weight: f64,
    pub data_index_weight: f64,
    pub performance_weight: f64,
    pub affinity_weight: f64,
    pub editing_weight: f64,
    pub video_quality_weight: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// API响应包装器
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub total: Option<u32>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Success".to_string(),
            total: None,
        }
    }

    pub fn success_with_total(data: T, total: u32) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Success".to_string(),
            total: Some(total),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message,
            total: None,
        }
    }
}

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub name: Option<String>,
    pub department: Option<String>,
    pub region: Option<String>,
    pub account_type: Option<String>,
    pub min_score: Option<f64>,
    pub max_score: Option<f64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}