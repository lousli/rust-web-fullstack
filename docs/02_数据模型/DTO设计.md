# 数据传输对象 (DTO) 设计

## 1. 前端到后端 DTO

### 1.1 医生信息导入 DTO

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DoctorImportDto {
    pub doctor_id: String,
    pub name: String,
    pub title: Option<String>,
    pub region: Option<String>,
    pub department: Option<String>,
    pub agency_name: Option<String>,
    pub agency_price: Option<f64>,
    pub total_followers: Option<i32>,
    pub total_likes: Option<i32>,
    pub total_works: Option<i32>,
    
    // 7天数据
    pub likes_7d: Option<i32>,
    pub followers_7d: Option<i32>,
    pub shares_7d: Option<i32>,
    pub comments_7d: Option<i32>,
    pub works_7d: Option<i32>,
    
    // 15天数据
    pub likes_15d: Option<i32>,
    pub followers_15d: Option<i32>,
    pub shares_15d: Option<i32>,
    pub comments_15d: Option<i32>,
    pub works_15d: Option<i32>,
    
    // 30天数据
    pub likes_30d: Option<i32>,
    pub followers_30d: Option<i32>,
    pub shares_30d: Option<i32>,
    pub comments_30d: Option<i32>,
    pub works_30d: Option<i32>,
    
    // 人工评分
    pub performance_score: Option<f32>,
    pub affinity_score: Option<f32>,
    pub editing_score: Option<f32>,
    pub video_quality_score: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct BatchImportDto {
    pub doctors: Vec<DoctorImportDto>,
    pub overwrite_existing: bool,
}
```

### 1.2 权重配置 DTO

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct WeightConfigDto {
    pub name: String,
    pub description: Option<String>,
    pub account_type_weight: f32,
    pub cost_effectiveness_weight: f32,
    pub data_trend_weight: f32,
    pub performance_weight: f32,
    pub affinity_weight: f32,
    pub editing_weight: f32,
    pub video_quality_weight: f32,
}

#[derive(Debug, Deserialize)]
pub struct WeightUpdateDto {
    pub id: i32,
    pub weights: WeightConfigDto,
}
```

### 1.3 查询参数 DTO

```rust
#[derive(Debug, Deserialize)]
pub struct DoctorQueryDto {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub region: Option<String>,
    pub department: Option<String>,
    pub account_type: Option<String>,
    pub min_score: Option<f32>,
    pub max_score: Option<f32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScoreRangeDto {
    pub min_comprehensive_score: Option<f32>,
    pub max_comprehensive_score: Option<f32>,
    pub min_cost_effectiveness: Option<f32>,
    pub max_cost_effectiveness: Option<f32>,
}
```

## 2. 后端到前端 DTO

### 2.1 医生详细信息 DTO

```rust
#[derive(Debug, Serialize)]
pub struct DoctorDetailDto {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub region: Option<String>,
    pub department: Option<String>,
    pub agency_name: Option<String>,
    pub agency_price: Option<f64>,
    
    // 基础数据
    pub total_followers: i32,
    pub total_likes: i32,
    pub total_works: i32,
    
    // 近期数据
    pub metrics: DoctorMetricsDto,
    
    // 人工评分
    pub scores: DoctorScoresDto,
    
    // 计算指标
    pub indicators: CalculatedIndicatorsDto,
    
    // 元数据
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct DoctorMetricsDto {
    pub likes_7d: i32,
    pub followers_7d: i32,
    pub shares_7d: i32,
    pub comments_7d: i32,
    pub works_7d: i32,
    
    pub likes_15d: i32,
    pub followers_15d: i32,
    pub shares_15d: i32,
    pub comments_15d: i32,
    pub works_15d: i32,
    
    pub likes_30d: i32,
    pub followers_30d: i32,
    pub shares_30d: i32,
    pub comments_30d: i32,
    pub works_30d: i32,
}

#[derive(Debug, Serialize)]
pub struct DoctorScoresDto {
    pub performance_score: Option<f32>,
    pub affinity_score: Option<f32>,
    pub editing_score: Option<f32>,
    pub video_quality_score: Option<f32>,
    pub scored_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CalculatedIndicatorsDto {
    pub account_type: String,
    pub account_type_score: f32,
    pub cost_effectiveness_index: f32,
    pub data_trend_index: f32,
    pub growth_stability_index: f32,
    pub content_quality_index: f32,
    pub comprehensive_index: f32,
    pub calculated_at: String,
}
```

### 2.2 医生列表 DTO

```rust
#[derive(Debug, Serialize)]
pub struct DoctorListDto {
    pub doctors: Vec<DoctorSummaryDto>,
    pub total_count: i32,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}

#[derive(Debug, Serialize)]
pub struct DoctorSummaryDto {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub region: Option<String>,
    pub department: Option<String>,
    pub agency_price: Option<f64>,
    pub total_followers: i32,
    pub account_type: String,
    pub comprehensive_index: f32,
    pub cost_effectiveness_index: f32,
    pub rank: Option<i32>,
}
```

### 2.3 统计分析 DTO

```rust
#[derive(Debug, Serialize)]
pub struct DoctorStatisticsDto {
    pub total_doctors: i32,
    pub head_account_count: i32,
    pub middle_account_count: i32,
    pub tail_account_count: i32,
    pub avg_comprehensive_score: f32,
    pub avg_cost_effectiveness: f32,
    pub score_distribution: Vec<ScoreDistributionDto>,
    pub region_distribution: Vec<RegionDistributionDto>,
    pub department_distribution: Vec<DepartmentDistributionDto>,
}

#[derive(Debug, Serialize)]
pub struct ScoreDistributionDto {
    pub score_range: String,
    pub count: i32,
    pub percentage: f32,
}

#[derive(Debug, Serialize)]
pub struct RegionDistributionDto {
    pub region: String,
    pub count: i32,
    pub avg_score: f32,
}

#[derive(Debug, Serialize)]
pub struct DepartmentDistributionDto {
    pub department: String,
    pub count: i32,
    pub avg_score: f32,
}
```

## 3. 错误响应 DTO

```rust
#[derive(Debug, Serialize)]
pub struct ErrorResponseDto {
    pub error_code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorDto {
    pub field: String,
    pub message: String,
    pub rejected_value: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ImportResultDto {
    pub total_records: i32,
    pub success_count: i32,
    pub failed_count: i32,
    pub errors: Vec<ImportErrorDto>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ImportErrorDto {
    pub row: i32,
    pub doctor_id: Option<String>,
    pub field: Option<String>,
    pub message: String,
}
```

## 4. 系统配置 DTO

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemConfigDto {
    pub account_head_threshold: i32,
    pub account_middle_threshold: i32,
    pub cost_base_price: f64,
    pub trend_weight_7d: f32,
    pub trend_weight_15d: f32,
    pub trend_weight_30d: f32,
}

#[derive(Debug, Serialize)]
pub struct WeightConfigListDto {
    pub configs: Vec<WeightConfigSummaryDto>,
    pub current_config_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct WeightConfigSummaryDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_default: bool,
    pub created_at: String,
    pub updated_at: String,
}
```

## 5. 数据验证

```rust
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ValidatedDoctorDto {
    #[validate(length(min = 1, max = 20))]
    pub doctor_id: String,
    
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    
    #[validate(range(min = 0.0))]
    pub agency_price: Option<f64>,
    
    #[validate(range(min = 0))]
    pub total_followers: Option<i32>,
    
    #[validate(range(min = 0.0, max = 10.0))]
    pub performance_score: Option<f32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ValidatedWeightConfigDto {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    
    #[validate(range(min = 0.0, max = 100.0))]
    pub account_type_weight: f32,
    
    #[validate(range(min = 0.0, max = 100.0))]
    pub cost_effectiveness_weight: f32,
    
    // ... 其他权重字段验证
    
    #[validate(custom = "validate_weight_sum")]
    pub _phantom: std::marker::PhantomData<()>,
}

fn validate_weight_sum(config: &ValidatedWeightConfigDto) -> Result<(), validator::ValidationError> {
    let sum = config.account_type_weight + 
              config.cost_effectiveness_weight + 
              /* ... 其他权重 */;
    
    if (sum - 100.0).abs() > 0.01 {
        return Err(validator::ValidationError::new("weight_sum_not_100"));
    }
    Ok(())
}
```
