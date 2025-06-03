use actix_web::{web, HttpResponse, Result};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use std::collections::HashMap;
use crate::models::{Doctor, WeightConfig, ScoreComponents};
use crate::algorithms::scoring::ScoringAlgorithm;
use crate::algorithms::basic_scoring::calculate_comprehensive_score;
// use crate::algorithms::{AlgorithmConfig};

/// 简化的医生评分计算
fn calculate_doctor_score(doctor: &Doctor, _weight_config: &WeightConfig) -> f64 {
    let mut score = 50.0; // 基础分
    
    // 基于粉丝量
    if doctor.total_followers > 1_000_000 {
        score += 30.0;
    } else if doctor.total_followers > 100_000 {
        score += 20.0;
    } else {
        score += 10.0;
    }
    
    // 基于互动率
    let engagement_rate = if doctor.total_followers > 0 {
        doctor.total_likes as f64 / doctor.total_followers as f64
    } else {
        0.0
    };
    score += (engagement_rate * 20.0).min(20.0);
    
    score.min(100.0)
}

/// 报告生成请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportRequest {
    pub report_type: String,              // 报告类型："overview", "ranking", "analysis", "comparison"
    pub filters: Option<ReportFilters>,   // 筛选条件
    pub weight_config_id: Option<i64>,    // 权重配置ID
    pub export_format: String,            // 导出格式："json", "csv", "pdf"
    pub custom_fields: Option<Vec<String>>, // 自定义字段
}

/// 报告筛选条件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportFilters {
    pub regions: Option<Vec<String>>,     // 地区筛选
    pub departments: Option<Vec<String>>, // 科室筛选
    pub titles: Option<Vec<String>>,      // 职称筛选
    pub institutions: Option<Vec<String>>, // 机构筛选
    pub score_range: Option<ScoreRange>,  // 评分范围
    pub fans_range: Option<NumberRange>,  // 粉丝数范围
    pub price_range: Option<NumberRange>, // 价格范围
    pub date_range: Option<DateRange>,    // 日期范围
}

/// 数值范围
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NumberRange {
    pub min: Option<i64>,
    pub max: Option<i64>,
}

/// 评分范围
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoreRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// 日期范围
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateRange {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
}

/// 报告响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportResponse {
    pub report_id: String,
    pub report_type: String,
    pub generated_at: DateTime<Utc>,
    pub data: ReportData,
    pub metadata: ReportMetadata,
}

/// 报告数据
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportData {
    pub summary: Option<ReportSummary>,
    pub rankings: Option<Vec<DoctorRanking>>,
    pub analysis: Option<ReportAnalysis>,
    pub comparisons: Option<Vec<DoctorComparison>>,
    pub doctors: Vec<DoctorReportData>,
}

/// 报告元数据
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub total_doctors: usize,
    pub filters_applied: ReportFilters,
    pub weight_config: Option<WeightConfig>,
    pub generation_time_ms: u128,
}

/// 报告摘要
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_doctors: usize,
    pub regions_count: usize,
    pub departments_count: usize,
    pub avg_score: f64,
    pub score_distribution: HashMap<String, usize>, // 评分分布
    pub top_regions: Vec<RegionStats>,
    pub top_departments: Vec<DepartmentStats>,
    pub price_stats: PriceStats,
    pub engagement_stats: EngagementStats,
}

/// 地区统计
#[derive(Debug, Serialize, Deserialize)]
pub struct RegionStats {
    pub region: String,
    pub doctor_count: usize,
    pub avg_score: f64,
    pub avg_price: f64,
}

/// 科室统计
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentStats {
    pub department: String,
    pub doctor_count: usize,
    pub avg_score: f64,
    pub avg_price: f64,
}

/// 价格统计
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceStats {
    pub avg_price: f64,
    pub median_price: f64,
    pub min_price: i64,
    pub max_price: i64,
    pub price_ranges: HashMap<String, usize>,
}

/// 互动统计
#[derive(Debug, Serialize, Deserialize)]
pub struct EngagementStats {
    pub total_fans: i64,
    pub total_likes: i64,
    pub avg_engagement_rate: f64,
    pub top_engagement_rate: f64,
}

/// 医生排名
#[derive(Debug, Serialize, Deserialize)]
pub struct DoctorRanking {
    pub rank: usize,
    pub doctor: DoctorReportData,
    pub score_components: ScoreComponents,
}

/// 报告分析
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportAnalysis {
    pub correlations: Vec<CorrelationAnalysis>,
    pub trends: Vec<TrendAnalysis>,
    pub insights: Vec<String>,
    pub recommendations: Vec<String>,
}

/// 相关性分析
#[derive(Debug, Serialize, Deserialize)]
pub struct CorrelationAnalysis {
    pub factor1: String,
    pub factor2: String,
    pub correlation: f64,
    pub description: String,
}

/// 趋势分析
#[derive(Debug, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub metric: String,
    pub trend: String, // "increasing", "decreasing", "stable"
    pub change_rate: f64,
    pub description: String,
}

/// 医生对比
#[derive(Debug, Serialize, Deserialize)]
pub struct DoctorComparison {
    pub doctor1: DoctorReportData,
    pub doctor2: DoctorReportData,
    pub comparison_metrics: ComparisonMetrics,
}

/// 对比指标
#[derive(Debug, Serialize, Deserialize)]
pub struct ComparisonMetrics {
    pub score_diff: f64,
    pub price_diff: i64,
    pub fans_diff: i64,
    pub engagement_diff: f64,
    pub strengths_weaknesses: HashMap<String, String>,
}

/// 医生报告数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoctorReportData {
    pub doctor: Doctor,
    pub comprehensive_score: f64,
    pub score_components: ScoreComponents,
    pub ranking_position: Option<usize>,
    pub percentile: f64,
    pub cost_efficiency: f64, // 性价比分数
}

/// 生成概览报告
pub async fn generate_overview_report(
    pool: web::Data<SqlitePool>,
    request: web::Json<ReportRequest>,
) -> Result<HttpResponse> {
    let start_time = std::time::Instant::now();
    
    // 获取权重配置
    let weight_config = get_weight_config(&pool, request.weight_config_id).await?;
    
    // 获取筛选后的医生数据
    let doctors = get_filtered_doctors(&pool, &request.filters).await?;    // 计算综合评分
    let _scoring_algorithm = ScoringAlgorithm::new(Default::default());
    let mut doctor_reports: Vec<DoctorReportData> = Vec::new();
    for doctor in doctors {
        // 创建一个简化的评分计算
        let comprehensive_score = calculate_doctor_score(&doctor, &weight_config);        let score_components = ScoreComponents {
            comprehensive_score,
            account_type_score: 0.0,
            cost_performance_score: 0.0,
            data_trend_score: 0.0,
            content_quality_score: 0.0,
        };
        
        doctor_reports.push(DoctorReportData {
            doctor,
            comprehensive_score,
            score_components,
            ranking_position: None,
            percentile: 0.0,
            cost_efficiency: 0.0,
        });
    }
      // 排序并分配排名
    doctor_reports.sort_by(|a, b| b.comprehensive_score.partial_cmp(&a.comprehensive_score).unwrap());
    let total_doctors = doctor_reports.len();
    for (index, report) in doctor_reports.iter_mut().enumerate() {
        report.ranking_position = Some(index + 1);
        report.percentile = ((total_doctors - index) as f64 / total_doctors as f64) * 100.0;
        
        // 计算性价比 = 综合评分 / (价格/10000)
        if report.doctor.agency_price.unwrap_or(0.0) > 0.0 {
            report.cost_efficiency = report.comprehensive_score / (report.doctor.agency_price.unwrap_or(1.0) / 10000.0);
        }
    }
    
    // 生成摘要
    let summary = generate_report_summary(&doctor_reports);
    
    let generation_time = start_time.elapsed();
    
    let report = ReportResponse {
        report_id: format!("overview_{}", chrono::Utc::now().timestamp()),
        report_type: "overview".to_string(),
        generated_at: chrono::Utc::now(),
        data: ReportData {
            summary: Some(summary),
            rankings: None,
            analysis: None,
            comparisons: None,
            doctors: doctor_reports.clone(),
        },
        metadata: ReportMetadata {
            total_doctors: doctor_reports.len(),
            filters_applied: request.filters.clone().unwrap_or_default(),
            weight_config: Some(weight_config),
            generation_time_ms: generation_time.as_millis(),
        },
    };
    
    Ok(HttpResponse::Ok().json(report))
}

/// 生成排名报告
pub async fn generate_ranking_report(
    pool: web::Data<SqlitePool>,
    request: web::Json<ReportRequest>,
) -> Result<HttpResponse> {
    let start_time = std::time::Instant::now();
    
    let weight_config = get_weight_config(&pool, request.weight_config_id).await?;
    let doctors = get_filtered_doctors(&pool, &request.filters).await?;
    
    let mut doctor_reports: Vec<DoctorReportData> = Vec::new();
    for doctor in doctors {
        let score_components = calculate_comprehensive_score(&doctor, &weight_config);
        let comprehensive_score = score_components.comprehensive_score;
        
        doctor_reports.push(DoctorReportData {
            doctor,
            comprehensive_score,
            score_components,
            ranking_position: None,
            percentile: 0.0,
            cost_efficiency: 0.0,
        });
    }
    
    // 排序
    doctor_reports.sort_by(|a, b| b.comprehensive_score.partial_cmp(&a.comprehensive_score).unwrap());
      // 生成排名列表
    let mut rankings: Vec<DoctorRanking> = Vec::new();
    for (index, report) in doctor_reports.iter().enumerate() {
        rankings.push(DoctorRanking {
            rank: index + 1,
            doctor: (*report).clone(),
            score_components: report.score_components.clone(),
        });
    }
    
    let generation_time = start_time.elapsed();
    
    let report = ReportResponse {
        report_id: format!("ranking_{}", chrono::Utc::now().timestamp()),
        report_type: "ranking".to_string(),
        generated_at: chrono::Utc::now(),
        data: ReportData {
            summary: None,
            rankings: Some(rankings),
            analysis: None,
            comparisons: None,
            doctors: doctor_reports.clone(),
        },
        metadata: ReportMetadata {
            total_doctors: doctor_reports.len(),
            filters_applied: request.filters.clone().unwrap_or_default(),
            weight_config: Some(weight_config),
            generation_time_ms: generation_time.as_millis(),
        },
    };
    
    Ok(HttpResponse::Ok().json(report))
}

/// 生成分析报告
pub async fn generate_analysis_report(
    pool: web::Data<SqlitePool>,
    request: web::Json<ReportRequest>,
) -> Result<HttpResponse> {
    let start_time = std::time::Instant::now();
    
    let weight_config = get_weight_config(&pool, request.weight_config_id).await?;
    let doctors = get_filtered_doctors(&pool, &request.filters).await?;
    
    let mut doctor_reports: Vec<DoctorReportData> = Vec::new();
    for doctor in doctors {
        let score_components = calculate_comprehensive_score(&doctor, &weight_config);
        let comprehensive_score = score_components.comprehensive_score;
        
        doctor_reports.push(DoctorReportData {
            doctor,
            comprehensive_score,
            score_components,
            ranking_position: None,
            percentile: 0.0,
            cost_efficiency: 0.0,
        });
    }
    
    // 生成分析
    let analysis = generate_analysis(&doctor_reports);
    
    let generation_time = start_time.elapsed();
    
    let report = ReportResponse {
        report_id: format!("analysis_{}", chrono::Utc::now().timestamp()),
        report_type: "analysis".to_string(),
        generated_at: chrono::Utc::now(),
        data: ReportData {
            summary: None,
            rankings: None,
            analysis: Some(analysis),
            comparisons: None,
            doctors: doctor_reports.clone(),
        },
        metadata: ReportMetadata {
            total_doctors: doctor_reports.len(),
            filters_applied: request.filters.clone().unwrap_or_default(),
            weight_config: Some(weight_config),
            generation_time_ms: generation_time.as_millis(),
        },
    };
    
    Ok(HttpResponse::Ok().json(report))
}

/// 获取权重配置
async fn get_weight_config(
    pool: &SqlitePool,
    config_id: Option<i64>,
) -> Result<WeightConfig, actix_web::Error> {
    let config = if let Some(id) = config_id {
        sqlx::query_as!(WeightConfig, "SELECT * FROM weight_configs WHERE id = ?", id)
            .fetch_optional(pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
            .ok_or_else(|| actix_web::error::ErrorNotFound("权重配置不存在"))?
    } else {
        sqlx::query_as!(WeightConfig, "SELECT * FROM weight_configs WHERE is_default = 1 LIMIT 1")
            .fetch_optional(pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
            .ok_or_else(|| actix_web::error::ErrorNotFound("未找到默认权重配置"))?
    };
    
    Ok(config)
}

/// 获取筛选后的医生数据
async fn get_filtered_doctors(
    pool: &SqlitePool,
    filters: &Option<ReportFilters>,
) -> Result<Vec<Doctor>, actix_web::Error> {
    let mut query = "SELECT * FROM doctors WHERE 1=1".to_string();
    let mut params: Vec<String> = Vec::new();
    
    if let Some(filters) = filters {
        if let Some(regions) = &filters.regions {
            if !regions.is_empty() {
                let placeholders = regions.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                query.push_str(&format!(" AND region IN ({})", placeholders));
                params.extend(regions.clone());
            }
        }
        
        if let Some(departments) = &filters.departments {
            if !departments.is_empty() {
                let placeholders = departments.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                query.push_str(&format!(" AND department IN ({})", placeholders));
                params.extend(departments.clone());
            }
        }
        
        if let Some(titles) = &filters.titles {
            if !titles.is_empty() {
                let placeholders = titles.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                query.push_str(&format!(" AND title IN ({})", placeholders));
                params.extend(titles.clone());
            }
        }
        
        if let Some(institutions) = &filters.institutions {
            if !institutions.is_empty() {
                let placeholders = institutions.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                query.push_str(&format!(" AND institution IN ({})", placeholders));
                params.extend(institutions.clone());
            }
        }
        
        if let Some(price_range) = &filters.price_range {
            if let Some(min) = price_range.min {                query.push_str(" AND agency_price >= ?");
                params.push(min.to_string());
            }
            if let Some(max) = price_range.max {
                query.push_str(" AND agency_price <= ?");
                params.push(max.to_string());
            }
        }
          if let Some(fans_range) = &filters.fans_range {
            if let Some(min) = fans_range.min {
                query.push_str(" AND total_followers >= ?");
                params.push(min.to_string());
            }
            if let Some(max) = fans_range.max {
                query.push_str(" AND total_followers <= ?");
                params.push(max.to_string());
            }
        }
    }
      // 构建动态查询
    let mut query_builder = sqlx::query_as::<_, Doctor>(&query);
    for param in params {
        query_builder = query_builder.bind(param);
    }
    let doctors = query_builder
        .fetch_all(pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(doctors)
}

/// 生成报告摘要
fn generate_report_summary(doctor_reports: &[DoctorReportData]) -> ReportSummary {
    let total_doctors = doctor_reports.len();
    let mut regions: HashMap<String, Vec<&DoctorReportData>> = HashMap::new();
    let mut departments: HashMap<String, Vec<&DoctorReportData>> = HashMap::new();
    let mut prices: Vec<i64> = Vec::new();
    let mut scores: Vec<f64> = Vec::new();
    let mut total_fans = 0i64;
    let mut total_likes = 0i64;
      for report in doctor_reports {
        // 地区分组
        if let Some(region) = &report.doctor.region {
            regions.entry(region.clone())
                .or_insert_with(Vec::new)
                .push(report);
        }
        
        // 科室分组
        if let Some(department) = &report.doctor.department {
            departments.entry(department.clone())
                .or_insert_with(Vec::new)
                .push(report);
        }
          prices.push(report.doctor.agency_price.unwrap_or(0.0) as i64);
        scores.push(report.comprehensive_score);
        total_fans += report.doctor.total_followers as i64;
        total_likes += report.doctor.total_likes as i64;
    }
    
    // 计算统计数据
    let avg_score = scores.iter().sum::<f64>() / scores.len() as f64;
    
    // 评分分布
    let mut score_distribution = HashMap::new();
    for score in &scores {
        let range = match *score {
            s if s >= 90.0 => "90-100",
            s if s >= 80.0 => "80-89",
            s if s >= 70.0 => "70-79",
            s if s >= 60.0 => "60-69",
            _ => "0-59",
        };
        *score_distribution.entry(range.to_string()).or_insert(0) += 1;
    }
      // 排序地区
    let mut top_regions: Vec<RegionStats> = regions.iter().map(|(region, reports)| {
        let avg_score = reports.iter().map(|r| r.comprehensive_score).sum::<f64>() / reports.len() as f64;
        let avg_price = reports.iter().map(|r| r.doctor.agency_price.unwrap_or(0.0)).sum::<f64>() / reports.len() as f64;
        
        RegionStats {
            region: region.clone(),
            doctor_count: reports.len(),
            avg_score,
            avg_price,
        }
    }).collect();
    top_regions.sort_by(|a, b| b.avg_score.partial_cmp(&a.avg_score).unwrap());
    top_regions.truncate(10);
      // 排序科室
    let mut top_departments: Vec<DepartmentStats> = departments.iter().map(|(department, reports)| {
        let avg_score = reports.iter().map(|r| r.comprehensive_score).sum::<f64>() / reports.len() as f64;
        let avg_price = reports.iter().map(|r| r.doctor.agency_price.unwrap_or(0.0)).sum::<f64>() / reports.len() as f64;
        
        DepartmentStats {
            department: department.clone(),
            doctor_count: reports.len(),
            avg_score,
            avg_price,
        }
    }).collect();
    top_departments.sort_by(|a, b| b.avg_score.partial_cmp(&a.avg_score).unwrap());
    top_departments.truncate(10);
    
    // 价格统计
    prices.sort();
    let avg_price = prices.iter().sum::<i64>() as f64 / prices.len() as f64;
    let median_price = if prices.len() % 2 == 0 {
        (prices[prices.len()/2 - 1] + prices[prices.len()/2]) as f64 / 2.0
    } else {
        prices[prices.len()/2] as f64
    };
    
    let mut price_ranges = HashMap::new();
    for price in &prices {
        let range = match *price {
            p if p >= 100000 => "100000+",
            p if p >= 50000 => "50000-99999",
            p if p >= 20000 => "20000-49999",
            p if p >= 10000 => "10000-19999",
            _ => "0-9999",
        };
        *price_ranges.entry(range.to_string()).or_insert(0) += 1;
    }
    
    let price_stats = PriceStats {
        avg_price,
        median_price,
        min_price: *prices.first().unwrap_or(&0),
        max_price: *prices.last().unwrap_or(&0),
        price_ranges,
    };
    
    // 互动统计
    let avg_engagement_rate = if total_doctors > 0 {
        (total_likes as f64 / total_fans as f64) * 100.0
    } else {
        0.0
    };
      let top_engagement_rate = doctor_reports.iter()
        .map(|r| if r.doctor.total_followers > 0 {
            (r.doctor.total_likes as f64 / r.doctor.total_followers as f64) * 100.0
        } else {
            0.0
        })
        .fold(0.0, f64::max);
    
    let engagement_stats = EngagementStats {
        total_fans,
        total_likes,
        avg_engagement_rate,
        top_engagement_rate,
    };
    
    ReportSummary {
        total_doctors,
        regions_count: regions.len(),
        departments_count: departments.len(),
        avg_score,
        score_distribution,
        top_regions,
        top_departments,
        price_stats,
        engagement_stats,
    }
}

/// 生成分析报告
fn generate_analysis(doctor_reports: &[DoctorReportData]) -> ReportAnalysis {
    let correlations = Vec::new();
    let trends = Vec::new();
    let mut insights = Vec::new();
    let mut recommendations = Vec::new();
      // 相关性分析
    // 暂时注释掉，等待相关算法函数实现
    /*
    correlations.push(CorrelationAnalysis {
        factor1: "粉丝数量".to_string(),
        factor2: "综合评分".to_string(),
        correlation: calculate_correlation(
            &doctor_reports.iter().map(|r| r.doctor.total_followers as f64).collect::<Vec<_>>(),
            &doctor_reports.iter().map(|r| r.comprehensive_score).collect::<Vec<_>>()
        ),
        description: "粉丝数量与综合评分的相关性".to_string(),
    });
    
    correlations.push(CorrelationAnalysis {
        factor1: "机构报价".to_string(),        factor2: "综合评分".to_string(),
        correlation: calculate_correlation(
            &doctor_reports.iter().map(|r| r.doctor.agency_price.unwrap_or(0.0)).collect::<Vec<_>>(),
            &doctor_reports.iter().map(|r| r.comprehensive_score).collect::<Vec<_>>()
        ),
        description: "机构报价与综合评分的相关性".to_string(),
    });
    */
    
    // 生成洞察
    let high_score_count = doctor_reports.iter().filter(|r| r.comprehensive_score >= 80.0).count();
    let high_score_percentage = (high_score_count as f64 / doctor_reports.len() as f64) * 100.0;
    
    insights.push(format!("{}%的医生综合评分达到80分以上", high_score_percentage as u32));
    
    let avg_price = doctor_reports.iter().map(|r| r.doctor.agency_price.unwrap_or(0.0)).sum::<f64>() / doctor_reports.len() as f64;
    insights.push(format!("平均机构报价为{:.0}元", avg_price / 100.0));
    
    // 生成建议
    if high_score_percentage < 20.0 {
        recommendations.push("建议加强医生培训，提升整体表现质量".to_string());
    }
    
    if avg_price > 50000.0 {
        recommendations.push("当前平均投放成本较高，建议优化投放策略".to_string());
    }
    
    recommendations.push("重点关注高性价比医生，优化投放组合".to_string());
    
    ReportAnalysis {
        correlations,
        trends,
        insights,
        recommendations,
    }
}

/// 计算相关系数
// 计算相关系数的辅助函数（暂时注释，预留用于高级分析）
/*
fn calculate_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }
    
    let n = x.len() as f64;
    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;
    
    let mut numerator = 0.0;
    let mut sum_sq_x = 0.0;
    let mut sum_sq_y = 0.0;
    
    for i in 0..x.len() {
        let diff_x = x[i] - mean_x;
        let diff_y = y[i] - mean_y;
        
        numerator += diff_x * diff_y;
        sum_sq_x += diff_x * diff_x;
        sum_sq_y += diff_y * diff_y;
    }
      let denominator = (sum_sq_x * sum_sq_y).sqrt();
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}
*/

/// 导出报告为CSV格式
pub async fn export_report_csv(
    pool: web::Data<SqlitePool>,
    request: web::Json<ReportRequest>,
) -> Result<HttpResponse> {
    let weight_config = get_weight_config(&pool, request.weight_config_id).await?;
    let doctors = get_filtered_doctors(&pool, &request.filters).await?;
    
    let mut csv_content = String::new();
    
    // CSV 标题行
    csv_content.push_str("排名,医生ID,姓名,职称,地区,科室,机构,粉丝数,获赞数,综合评分,影响力评分,质量评分,性价比评分\n");
    
    let mut doctor_reports: Vec<DoctorReportData> = Vec::new();
    for doctor in doctors {
        let score_components = calculate_comprehensive_score(&doctor, &weight_config);
        let comprehensive_score = score_components.comprehensive_score;
          let cost_efficiency = if doctor.agency_price.unwrap_or(0.0) > 0.0 {
            comprehensive_score / (doctor.agency_price.unwrap_or(1.0) / 10000.0)
        } else {
            0.0
        };
        
        doctor_reports.push(DoctorReportData {
            doctor,
            comprehensive_score,
            score_components,
            ranking_position: None,
            percentile: 0.0,
            cost_efficiency,
        });
    }
    
    // 排序
    doctor_reports.sort_by(|a, b| b.comprehensive_score.partial_cmp(&a.comprehensive_score).unwrap());
      // 写入数据行
    for (index, report) in doctor_reports.iter().enumerate() {
        csv_content.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{:.2},{:.2},{:.2},{:.2}\n",
            index + 1,
            report.doctor.id,
            report.doctor.name,
            report.doctor.title.as_ref().unwrap_or(&"".to_string()),
            report.doctor.region.as_ref().unwrap_or(&"".to_string()),
            report.doctor.department.as_ref().unwrap_or(&"".to_string()),
            report.doctor.agency_name.as_ref().unwrap_or(&"".to_string()),
            report.doctor.total_followers,
            report.doctor.total_likes,
            report.comprehensive_score,
            report.score_components.account_type_score,
            report.score_components.content_quality_score,
            report.cost_efficiency
        ));
    }
    
    Ok(HttpResponse::Ok()
        .content_type("text/csv; charset=utf-8")
        .append_header(("Content-Disposition", "attachment; filename=\"doctor_report.csv\""))
        .body(csv_content))
}

impl Default for ReportFilters {
    fn default() -> Self {
        Self {
            regions: None,
            departments: None,
            titles: None,
            institutions: None,
            score_range: None,
            fans_range: None,
            price_range: None,
            date_range: None,
        }
    }
}
