use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

/// 统一API响应格式
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "操作成功".to_string(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    pub fn success_with_total(data: T, _total: i64) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "操作成功".to_string(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

/// 医生基础信息模型
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Doctor {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub region: Option<String>,
    pub department: Option<String>,
    pub agency_name: Option<String>,
    pub agency_price: Option<f64>,
    pub total_followers: i32,
    pub total_likes: i32,
    pub total_works: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// 医生数据指标模型
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct DoctorMetrics {
    pub id: Option<i64>,
    pub doctor_id: String,
    
    // 7天数据
    pub likes_7d: i32,
    pub followers_7d: i32,
    pub shares_7d: i32,
    pub comments_7d: i32,
    pub works_7d: i32,
    
    // 15天数据
    pub likes_15d: i32,
    pub followers_15d: i32,
    pub shares_15d: i32,
    pub comments_15d: i32,
    pub works_15d: i32,
    
    // 30天数据
    pub likes_30d: i32,
    pub followers_30d: i32,
    pub shares_30d: i32,
    pub comments_30d: i32,
    pub works_30d: i32,
    
    pub recorded_at: Option<DateTime<Utc>>,
}

/// 人工评分模型
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct DoctorScores {
    pub id: Option<i64>,
    pub doctor_id: String,
    pub performance_score: Option<f32>,    // 医生表现力评分 (0-10)
    pub affinity_score: Option<f32>,       // 医生亲和力评分 (0-10)
    pub editing_score: Option<f32>,        // 剪辑水平评分 (0-10)
    pub video_quality_score: Option<f32>,  // 画面质量评分 (0-10)
    pub scorer_id: Option<String>,
    pub scored_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// 计算指标模型
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CalculatedIndicators {
    pub id: Option<i64>,
    pub doctor_id: String,
    
    // 账号性质分类
    pub account_type: String,               // head/middle/tail
    pub account_type_score: f32,            // 账号性质评分 (0-100)
    
    // 性价比指数
    pub cost_effectiveness_index: f32,      // 性价比指数 (0-100)
    
    // 数据指数
    pub data_trend_index: f32,              // 数据趋势指数 (0-100)
    pub growth_stability_index: f32,        // 增长稳定性指数 (0-100)
    
    // 内容质量指数
    pub content_quality_index: f32,         // 内容质量指数 (0-100)
    
    // 综合指数
    pub comprehensive_index: f32,           // 综合评价指数 (0-100)
    
    pub calculated_at: Option<DateTime<Utc>>,
    pub weight_config_id: Option<i32>,
}

/// 权重配置模型
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct WeightConfig {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    
    // 各项指标权重 (总和为100)
    pub account_type_weight: f32,
    pub cost_effectiveness_weight: f32,
    pub data_trend_weight: f32,
    pub performance_weight: f32,
    pub affinity_weight: f32,
    pub editing_weight: f32,
    pub video_quality_weight: f32,
    
    pub is_default: bool,
    pub created_by: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// 系统配置模型
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct SystemConfig {
    pub id: Option<i32>,
    pub config_key: String,
    pub config_value: String,
    pub description: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// 医生导入数据模型
#[derive(Debug, Deserialize)]
pub struct DoctorImport {
    #[serde(alias = "姓名")]
    pub name: String,
    #[serde(alias = "职称")]
    pub title: Option<String>,
    #[serde(alias = "地区")]
    pub region: Option<String>,
    #[serde(alias = "科室")]
    pub department: Option<String>,
    #[serde(alias = "机构名称")]
    pub agency_name: Option<String>,
    #[serde(alias = "机构报价")]
    pub agency_price: Option<f64>,
    #[serde(alias = "总粉丝量")]
    pub total_followers: Option<i32>,
    #[serde(alias = "总获赞量")]
    pub total_likes: Option<i32>,
    #[serde(alias = "总作品数")]
    pub total_works: Option<i32>,
    
    // 7天数据
    #[serde(alias = "7天新增点赞")]
    pub likes_7d: Option<i32>,
    #[serde(alias = "7天净增粉丝")]
    pub followers_7d: Option<i32>,
    #[serde(alias = "7天新增分享")]
    pub shares_7d: Option<i32>,
    #[serde(alias = "7天新增评论")]
    pub comments_7d: Option<i32>,
    #[serde(alias = "7天新增作品")]
    pub works_7d: Option<i32>,
    
    // 15天数据
    #[serde(alias = "15天新增点赞")]
    pub likes_15d: Option<i32>,
    #[serde(alias = "15天净增粉丝")]
    pub followers_15d: Option<i32>,
    #[serde(alias = "15天新增分享")]
    pub shares_15d: Option<i32>,
    #[serde(alias = "15天新增评论")]
    pub comments_15d: Option<i32>,
    #[serde(alias = "15天新增作品")]
    pub works_15d: Option<i32>,
    
    // 30天数据
    #[serde(alias = "30天新增点赞")]
    pub likes_30d: Option<i32>,
    #[serde(alias = "30天净增粉丝")]
    pub followers_30d: Option<i32>,
    #[serde(alias = "30天新增分享")]
    pub shares_30d: Option<i32>,
    #[serde(alias = "30天新增评论")]
    pub comments_30d: Option<i32>,
    #[serde(alias = "30天新增作品")]
    pub works_30d: Option<i32>,
    
    // 人工评分
    #[serde(alias = "医生表现力评分")]
    pub performance_score: Option<f32>,
    #[serde(alias = "医生亲和力评分")]
    pub affinity_score: Option<f32>,
    #[serde(alias = "剪辑水平评分")]
    pub editing_score: Option<f32>,
    #[serde(alias = "画面质量评分")]
    pub video_quality_score: Option<f32>,
}

/// 医生完整信息 DTO
#[derive(Debug, Serialize)]
pub struct DoctorDetailDto {
    #[serde(flatten)]
    pub doctor: Doctor,
    pub metrics: Option<DoctorMetrics>,
    pub scores: Option<DoctorScores>,
    pub indicators: Option<CalculatedIndicators>,
}

/// 医生列表项 DTO
#[derive(Debug, Serialize)]
pub struct DoctorSummaryDto {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub region: Option<String>,
    pub department: Option<String>,
    pub agency_price: Option<f64>,
    pub total_followers: i32,
    pub account_type: Option<String>,
    pub comprehensive_index: Option<f32>,
    pub cost_effectiveness_index: Option<f32>,
    pub rank: Option<i32>,
}

/// 分页信息
#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: i32,
    pub page_size: i32,
    pub total_count: i64,
    pub total_pages: i32,
}

/// 医生列表响应
#[derive(Debug, Serialize)]
pub struct DoctorListResponse {
    pub doctors: Vec<DoctorSummaryDto>,
    pub pagination: PaginationInfo,
}

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct DoctorQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub region: Option<String>,
    pub department: Option<String>,
    pub account_type: Option<String>,
    pub min_score: Option<f32>,
    pub max_score: Option<f32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search: Option<String>,
}

/// 账号类型枚举
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccountType {
    #[serde(rename = "head")]
    Head,    // 头部账号
    #[serde(rename = "middle")]
    Middle,  // 腰部账号
    #[serde(rename = "tail")]
    Tail,    // 尾部账号
}

impl AccountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountType::Head => "head",
            AccountType::Middle => "middle",
            AccountType::Tail => "tail",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "head" => Some(AccountType::Head),
            "middle" => Some(AccountType::Middle),
            "tail" => Some(AccountType::Tail),
            _ => None,
        }
    }
}

/// 分析参数 (保持向后兼容)
#[derive(Debug, Deserialize)]
pub struct AnalysisParams {
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub weight_config_id: Option<i32>,
    pub department: Option<String>,
    pub region: Option<String>,
    pub title: Option<String>,
}

/// 查询参数模型
#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub weight_config_id: Option<i32>,
    pub department: Option<String>,
    pub region: Option<String>,
    pub title: Option<String>,
    pub name: Option<String>,
    pub account_type: Option<String>,
    pub min_score: Option<f64>,
    pub max_score: Option<f64>,
}

/// 向后兼容的医生评分模型
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct DoctorScore {
    pub id: Option<i64>,
    pub doctor_id: String,
    pub doctor_name: Option<String>,
    pub department: Option<String>,
    pub region: Option<String>,
    pub institution: Option<String>,
    pub account_type: Option<String>,
    pub influence_score: f64,
    pub quality_score: f64,
    pub activity_score: f64,
    pub comprehensive_score: f64,
    pub cost_performance_index: f64,
    pub cost_performance_score: Option<f64>,
    pub data_index_score: Option<f64>,
    pub performance_score: Option<f64>,
    pub affinity_score: Option<f64>,
    pub editing_score: Option<f64>,
    pub video_quality_score: Option<f64>,
    pub weighted_total_score: f64,
    pub ranking: Option<i64>,
    pub weight_config_id: i32,
    pub calculated_at: Option<DateTime<Utc>>,
}

impl Default for WeightConfig {
    fn default() -> Self {
        Self {
            id: None,
            name: "默认配置".to_string(),
            description: Some("系统默认权重配置".to_string()),
            account_type_weight: 25.0,
            cost_effectiveness_weight: 30.0,
            data_trend_weight: 25.0,
            performance_weight: 6.0,
            affinity_weight: 5.0,
            editing_weight: 5.0,
            video_quality_weight: 4.0,
            is_default: true,
            created_by: Some("system".to_string()),
            created_at: None,
            updated_at: None,
        }
    }
}