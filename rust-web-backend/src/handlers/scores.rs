use actix_web::{web, HttpResponse, Result};
use sqlx::{SqlitePool, Row};
use chrono::{Utc, DateTime};
use crate::models::{WeightConfig, DoctorScore, ApiResponse, Doctor};
use crate::algorithms::ScoringAlgorithm;
use std::collections::HashMap;

/// 分析参数（保留用于复杂查询功能扩展）
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct AnalysisParams {
    pub weight_config_id: Option<i64>,
    pub doctor_ids: Option<Vec<String>>,
    pub region: Option<String>,
    pub department: Option<String>,
    pub title: Option<String>,
}

/// 查询参数
#[derive(Debug, serde::Deserialize)]
pub struct QueryParams {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub name: Option<String>,
    pub department: Option<String>,
    pub region: Option<String>,
    pub account_type: Option<String>,
    pub min_score: Option<f64>,
    pub max_score: Option<f64>,
}

/// 计算医生评分
pub async fn calculate_scores(
    pool: web::Data<SqlitePool>,
    params: web::Query<AnalysisParams>,
) -> Result<HttpResponse> {
    // 获取权重配置
    let weight_config_id = params.weight_config_id.unwrap_or(1);
    let weight_config: Option<WeightConfig> = sqlx::query_as(
        "SELECT * FROM weight_configs WHERE id = ?"
    )
    .bind(weight_config_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let weight_config = match weight_config {
        Some(config) => config,
        None => {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "权重配置不存在".to_string()
            )));
        }
    };

    // 构建医生查询条件
    let mut where_conditions = Vec::new();
    if let Some(department) = &params.department {
        where_conditions.push(format!("department = '{}'", department));
    }
    if let Some(region) = &params.region {
        where_conditions.push(format!("region = '{}'", region));
    }
    if let Some(title) = &params.title {
        where_conditions.push(format!("title = '{}'", title));
    }

    let where_clause = if where_conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_conditions.join(" AND "))
    };

    // 获取医生数据
    let sql = format!("SELECT * FROM doctors {}", where_clause);
    let doctors: Vec<Doctor> = sqlx::query_as(&sql)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if doctors.is_empty() {
        return Ok(HttpResponse::Ok().json(ApiResponse::success(Vec::<DoctorScore>::new())));
    }    // 计算评分
    let scores = ScoringAlgorithm::calculate_scores(&doctors, &weight_config);

    // 删除旧的评分记录
    sqlx::query("DELETE FROM doctor_scores WHERE weight_config_id = ?")
        .bind(weight_config_id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 保存新的评分记录
    for score in &scores {
        sqlx::query(
            r#"
            INSERT INTO doctor_scores (
                doctor_id, weight_config_id, influence_score, quality_score, activity_score,
                comprehensive_score, cost_performance_index, ranking
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&score.doctor_id)
        .bind(score.weight_config_id)
        .bind(score.influence_score)
        .bind(score.quality_score)
        .bind(score.activity_score)
        .bind(score.comprehensive_score)
        .bind(score.cost_performance_index)
        .bind(score.ranking)
        .execute(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(scores)))
}

/// 获取医生排名
pub async fn get_ranking(
    pool: web::Data<SqlitePool>,
    params: web::Query<AnalysisParams>,
) -> Result<HttpResponse> {
    let weight_config_id = params.weight_config_id.unwrap_or(1);

    let ranking_data: Vec<(String, String, String, String, Option<String>, f64, f64, f64, f64, Option<i64>)> = sqlx::query_as(
        r#"
        SELECT 
            d.id, d.name, d.title, d.department, d.institution,
            ds.influence_score, ds.quality_score, ds.comprehensive_score, 
            ds.cost_performance_index, ds.ranking
        FROM doctors d
        LEFT JOIN doctor_scores ds ON d.id = ds.doctor_id AND ds.weight_config_id = ?
        ORDER BY ds.comprehensive_score DESC NULLS LAST
        "#,
    )
    .bind(weight_config_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let result: Vec<HashMap<String, serde_json::Value>> = ranking_data
        .into_iter()
        .map(|(id, name, title, department, institution, influence, quality, comprehensive, cpi, ranking)| {
            let mut map = HashMap::new();
            map.insert("id".to_string(), serde_json::Value::String(id));
            map.insert("name".to_string(), serde_json::Value::String(name));
            map.insert("title".to_string(), serde_json::Value::String(title));
            map.insert("department".to_string(), serde_json::Value::String(department));
            map.insert("institution".to_string(), serde_json::Value::String(institution.unwrap_or_default()));
            map.insert("influence_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(influence).unwrap()));
            map.insert("quality_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(quality).unwrap()));
            map.insert("comprehensive_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(comprehensive).unwrap()));
            map.insert("cost_performance_index".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(cpi).unwrap()));
            map.insert("ranking".to_string(), match ranking {
                Some(r) => serde_json::Value::Number(serde_json::Number::from(r)),
                None => serde_json::Value::Null,
            });
            map
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

/// 医生对比分析
pub async fn compare_doctors(
    pool: web::Data<SqlitePool>,
    doctor_ids: web::Json<Vec<String>>,
) -> Result<HttpResponse> {
    let ids = doctor_ids.into_inner();
    
    if ids.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "请选择要对比的医生".to_string()
        )));
    }

    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        r#"
        SELECT 
            d.*, 
            ds.influence_score, ds.quality_score, ds.activity_score,
            ds.comprehensive_score, ds.cost_performance_index, ds.ranking
        FROM doctors d
        LEFT JOIN doctor_scores ds ON d.id = ds.doctor_id
        WHERE d.id IN ({})
        "#,
        placeholders
    );

    let mut query = sqlx::query(&sql);
    for id in &ids {
        query = query.bind(id);
    }

    let rows = query
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut result = Vec::new();
    for row in rows {
        let mut doctor_data = HashMap::new();
        
        // 基础信息
        doctor_data.insert("id", row.get::<String, _>("id"));
        doctor_data.insert("name", row.get::<String, _>("name"));
        doctor_data.insert("title", row.get::<String, _>("title"));
        doctor_data.insert("department", row.get::<String, _>("department"));
        doctor_data.insert("region", row.get::<String, _>("region"));
        
        // 数据指标
        doctor_data.insert("total_fans", row.get::<i64, _>("total_fans").to_string());
        doctor_data.insert("total_likes", row.get::<i64, _>("total_likes").to_string());
        doctor_data.insert("agency_price", row.get::<Option<f64>, _>("agency_price").unwrap_or(0.0).to_string());
        
        // 评分数据
        if let Ok(score) = row.try_get::<f64, _>("influence_score") {
            doctor_data.insert("influence_score", score.to_string());
        }
        if let Ok(score) = row.try_get::<f64, _>("quality_score") {
            doctor_data.insert("quality_score", score.to_string());
        }
        if let Ok(score) = row.try_get::<f64, _>("comprehensive_score") {
            doctor_data.insert("comprehensive_score", score.to_string());
        }
        if let Ok(cpi) = row.try_get::<f64, _>("cost_performance_index") {
            doctor_data.insert("cost_performance_index", cpi.to_string());
        }
        
        result.push(doctor_data);
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

/// 获取投放建议
pub async fn get_recommendations(
    pool: web::Data<SqlitePool>,
    params: web::Query<AnalysisParams>,
) -> Result<HttpResponse> {
    let weight_config_id = params.weight_config_id.unwrap_or(1);
    let limit = 10; // 默认推荐前10名

    // 获取医生数据和评分
    let doctors: Vec<Doctor> = sqlx::query_as("SELECT * FROM doctors")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let scores: Vec<DoctorScore> = sqlx::query_as(
        "SELECT * FROM doctor_scores WHERE weight_config_id = ? ORDER BY cost_performance_index DESC LIMIT ?"
    )
    .bind(weight_config_id)
    .bind(limit as i32)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;    let recommendations = ScoringAlgorithm::get_investment_recommendations(&scores, &doctors, limit);

    let result: Vec<serde_json::Value> = recommendations
        .into_iter()
        .enumerate()
        .map(|(index, mut recommendation)| {
            if let serde_json::Value::Object(ref mut map) = recommendation {
                map.insert("rank".to_string(), serde_json::Value::Number(serde_json::Number::from(index + 1)));
            }
            recommendation
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

/// 获取趋势分析数据（预留用于时间序列分析功能）
#[allow(dead_code)]
pub async fn get_trends(
    pool: web::Data<SqlitePool>,
    _params: web::Query<AnalysisParams>,
) -> Result<HttpResponse> {
    // 这里可以实现基于时间的趋势分析
    // 目前返回一个模拟的趋势数据
    let mut trends = HashMap::new();
    
    // 粉丝增长趋势
    let fan_growth: Vec<(String, i64)> = sqlx::query_as(
        "SELECT department, AVG(followers_15d) as avg_growth FROM doctors GROUP BY department ORDER BY avg_growth DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 内容产出趋势
    let content_production: Vec<(String, i64)> = sqlx::query_as(
        "SELECT department, AVG(works_7d) as avg_works FROM doctors GROUP BY department ORDER BY avg_works DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    trends.insert("fan_growth_by_department", serde_json::to_value(fan_growth).unwrap());
    trends.insert("content_production_by_department", serde_json::to_value(content_production).unwrap());

    Ok(HttpResponse::Ok().json(ApiResponse::success(trends)))
}

/// 模拟医生评分数据
fn get_mock_doctor_scores() -> Vec<DoctorScore> {
    vec![
        DoctorScore {
            id: Some(1),
            doctor_id: "doc_001".to_string(),
            doctor_name: Some("张伟明".to_string()),
            department: Some("心内科".to_string()),
            region: Some("北京".to_string()),
            institution: Some("北京协和医院".to_string()),
            account_type: Some("头部".to_string()),
            influence_score: 85.0,
            quality_score: 88.5,
            activity_score: 82.3,
            comprehensive_score: 91.6,
            cost_performance_index: 92.5,
            cost_performance_score: Some(92.5),
            data_index_score: Some(89.3),
            performance_score: Some(94.8),
            affinity_score: Some(91.2),
            editing_score: Some(88.7),
            video_quality_score: Some(93.1),
            weighted_total_score: 91.6,
            ranking: Some(1),
            weight_config_id: 1,
            calculated_at: Some(Utc::now()),
        },
        DoctorScore {
            id: Some(2),
            doctor_id: "doc_002".to_string(),
            doctor_name: Some("李小红".to_string()),
            department: Some("神经内科".to_string()),
            region: Some("上海".to_string()),
            institution: Some("上海华山医院".to_string()),
            account_type: Some("腰部".to_string()),
            influence_score: 80.0,
            quality_score: 83.5,
            activity_score: 78.8,
            comprehensive_score: 85.9,
            cost_performance_index: 85.2,
            cost_performance_score: Some(85.2),
            data_index_score: Some(87.6),
            performance_score: Some(82.4),
            affinity_score: Some(89.1),
            editing_score: Some(84.3),
            video_quality_score: Some(86.8),
            weighted_total_score: 85.9,
            ranking: Some(2),
            weight_config_id: 1,
            calculated_at: Some(Utc::now()),
        },
        DoctorScore {
            id: Some(3),
            doctor_id: "doc_003".to_string(),
            doctor_name: Some("王建国".to_string()),
            department: Some("骨科".to_string()),
            region: Some("广州".to_string()),
            institution: Some("中山大学附属第一医院".to_string()),
            account_type: Some("尾部".to_string()),
            influence_score: 75.0,
            quality_score: 76.2,
            activity_score: 74.1,
            comprehensive_score: 77.7,
            cost_performance_index: 78.9,
            cost_performance_score: Some(78.9),
            data_index_score: Some(76.4),
            performance_score: Some(79.2),
            affinity_score: Some(77.8),
            editing_score: Some(75.6),
            video_quality_score: Some(78.1),
            weighted_total_score: 77.7,
            ranking: Some(3),
            weight_config_id: 1,
            calculated_at: Some(Utc::now()),
        },
        DoctorScore {
            id: Some(4),
            doctor_id: "doc_004".to_string(),
            doctor_name: Some("陈雅文".to_string()),
            department: Some("妇产科".to_string()),
            region: Some("深圳".to_string()),
            institution: Some("深圳市人民医院".to_string()),
            account_type: Some("头部".to_string()),
            influence_score: 90.0,
            quality_score: 93.2,
            activity_score: 88.7,
            comprehensive_score: 94.4,
            cost_performance_index: 95.1,
            cost_performance_score: Some(95.1),
            data_index_score: Some(92.8),
            performance_score: Some(96.3),
            affinity_score: Some(94.7),
            editing_score: Some(91.5),
            video_quality_score: Some(95.9),
            weighted_total_score: 94.4,
            ranking: Some(1),
            weight_config_id: 1,
            calculated_at: Some(Utc::now()),
        },
        DoctorScore {
            id: Some(5),
            doctor_id: "doc_005".to_string(),
            doctor_name: Some("刘志强".to_string()),
            department: Some("儿科".to_string()),
            region: Some("杭州".to_string()),
            institution: Some("浙江大学医学院附属儿童医院".to_string()),
            account_type: Some("腰部".to_string()),
            influence_score: 82.0,
            quality_score: 85.4,
            activity_score: 80.3,
            comprehensive_score: 87.9,
            cost_performance_index: 88.3,
            cost_performance_score: Some(88.3),
            data_index_score: Some(85.7),
            performance_score: Some(87.9),
            affinity_score: Some(90.4),
            editing_score: Some(86.2),
            video_quality_score: Some(88.8),
            weighted_total_score: 87.9,
            ranking: Some(2),
            weight_config_id: 1,
            calculated_at: Some(Utc::now()),
        },
        DoctorScore {
            id: Some(6),
            doctor_id: "doc_006".to_string(),
            doctor_name: Some("赵美丽".to_string()),
            department: Some("眼科".to_string()),
            region: Some("成都".to_string()),
            institution: Some("四川大学华西医院".to_string()),
            account_type: Some("尾部".to_string()),
            influence_score: 70.0,
            quality_score: 72.8,
            activity_score: 71.2,
            comprehensive_score: 73.4,
            cost_performance_index: 72.6,
            cost_performance_score: Some(72.6),
            data_index_score: Some(74.1),
            performance_score: Some(71.8),
            affinity_score: Some(75.3),
            editing_score: Some(73.7),
            video_quality_score: Some(72.9),
            weighted_total_score: 73.4,
            ranking: Some(4),
            weight_config_id: 1,
            calculated_at: Some(Utc::now()),
        },
        DoctorScore {
            id: Some(7),
            doctor_id: "doc_007".to_string(),
            doctor_name: Some("孙立华".to_string()),
            department: Some("呼吸内科".to_string()),
            region: Some("西安".to_string()),
            institution: Some("西安交通大学第一附属医院".to_string()),
            account_type: Some("头部".to_string()),
            influence_score: 87.0,
            quality_score: 90.1,
            activity_score: 85.5,
            comprehensive_score: 90.0,
            cost_performance_index: 90.7,
            cost_performance_score: Some(90.7),
            data_index_score: Some(88.9),
            performance_score: Some(92.1),
            affinity_score: Some(89.6),
            editing_score: Some(87.4),
            video_quality_score: Some(91.3),
            weighted_total_score: 90.0,
            ranking: Some(1),
            weight_config_id: 1,
            calculated_at: Some(Utc::now()),
        },
        DoctorScore {
            id: Some(8),
            doctor_id: "doc_008".to_string(),
            doctor_name: Some("周晓明".to_string()),
            department: Some("消化内科".to_string()),
            region: Some("武汉".to_string()),
            institution: Some("华中科技大学同济医学院附属同济医院".to_string()),
            account_type: Some("腰部".to_string()),
            influence_score: 78.0,
            quality_score: 81.6,
            activity_score: 77.9,
            comprehensive_score: 82.8,
            cost_performance_index: 83.4,
            cost_performance_score: Some(83.4),
            data_index_score: Some(81.7),
            performance_score: Some(84.2),
            affinity_score: Some(82.9),
            editing_score: Some(80.8),
            video_quality_score: Some(83.6),
            weighted_total_score: 82.8,
            ranking: Some(3),
            weight_config_id: 1,
            calculated_at: Some(Utc::now()),
        },
    ]
}

/// 获取医生评分列表
pub async fn get_scores(query: web::Query<QueryParams>) -> Result<HttpResponse> {
    let mut scores = get_mock_doctor_scores();
      // 应用筛选条件
    if let Some(ref name) = query.name {
        scores.retain(|s| s.doctor_name.as_ref().map_or(false, |n| n.contains(name)));
    }
    
    if let Some(ref department) = query.department {
        scores.retain(|s| s.department.as_ref() == Some(department));
    }
    
    if let Some(ref region) = query.region {
        scores.retain(|s| s.region.as_ref() == Some(region));
    }
    
    if let Some(ref account_type) = query.account_type {
        scores.retain(|s| s.account_type.as_ref() == Some(account_type));
    }
    
    if let Some(min_score) = query.min_score {
        scores.retain(|s| s.weighted_total_score >= min_score);
    }
    
    if let Some(max_score) = query.max_score {
        scores.retain(|s| s.weighted_total_score <= max_score);
    }
    
    // 排序
    if let Some(ref sort_by) = query.sort_by {
        let ascending = query.sort_order.as_deref() != Some("desc");
        
        match sort_by.as_str() {
            "total_score" => {
                scores.sort_by(|a, b| {
                    if ascending {
                        a.weighted_total_score.partial_cmp(&b.weighted_total_score).unwrap()
                    } else {
                        b.weighted_total_score.partial_cmp(&a.weighted_total_score).unwrap()
                    }
                });
            }            "name" => {
                scores.sort_by(|a, b| {
                    let a_name = a.doctor_name.as_deref().unwrap_or("");
                    let b_name = b.doctor_name.as_deref().unwrap_or("");
                    if ascending {
                        a_name.cmp(b_name)
                    } else {
                        b_name.cmp(a_name)
                    }
                });
            }
            _ => {}
        }
    } else {
        // 默认按总分降序排列
        scores.sort_by(|a, b| b.weighted_total_score.partial_cmp(&a.weighted_total_score).unwrap());
    }
    
    let total = scores.len() as i64;
      // 分页
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let start = ((page - 1) * page_size) as usize;
    let end = (start + page_size as usize).min(scores.len());
    
    let paginated_scores = if start < scores.len() {
        scores[start..end].to_vec()
    } else {
        vec![]
    };
    
    Ok(HttpResponse::Ok().json(ApiResponse::success_with_total(paginated_scores, total)))
}

/// 获取单个医生的评分详情
pub async fn get_doctor_score(path: web::Path<String>) -> Result<HttpResponse> {
    let doctor_id = path.into_inner();
    let scores = get_mock_doctor_scores();
    
    if let Some(score) = scores.into_iter().find(|s| s.doctor_id == doctor_id) {
        Ok(HttpResponse::Ok().json(ApiResponse::success(score)))
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("医生评分未找到".to_string())))
    }
}

/// 获取评分统计数据
pub async fn get_score_statistics() -> Result<HttpResponse> {
    let scores = get_mock_doctor_scores();
    
    let total_count = scores.len();
    let avg_score = scores.iter().map(|s| s.weighted_total_score).sum::<f64>() / total_count as f64;
    let max_score = scores.iter().map(|s| s.weighted_total_score).fold(0.0, f64::max);
    let min_score = scores.iter().map(|s| s.weighted_total_score).fold(100.0, f64::min);
      // 按科室统计
    let mut dept_stats = std::collections::HashMap::new();
    for score in &scores {
        if let Some(ref department) = score.department {
            let entry = dept_stats.entry(department.clone()).or_insert(vec![]);
            entry.push(score.weighted_total_score);
        }
    }
    
    let dept_averages: Vec<_> = dept_stats.iter().map(|(dept, scores)| {
        let avg = scores.iter().sum::<f64>() / scores.len() as f64;
        serde_json::json!({
            "department": dept,
            "average_score": avg,
            "count": scores.len()
        })
    }).collect();
    
    let statistics = serde_json::json!({
        "total_doctors": total_count,
        "average_score": avg_score,
        "max_score": max_score,
        "min_score": min_score,
        "department_stats": dept_averages
    });
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(statistics)))
}

/// 获取医生评分趋势
pub async fn get_score_trends(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let doctor_id = path.into_inner();

    // 获取医生历史评分记录
    let trends: Vec<(String, f64, f64, f64, f64, DateTime<Utc>)> = sqlx::query_as(
        r#"
        SELECT 
            doctor_id, 
            comprehensive_score, 
            cost_performance_index, 
            influence_score, 
            quality_score,
            calculated_at
        FROM doctor_scores 
        WHERE doctor_id = ? 
        ORDER BY calculated_at DESC 
        LIMIT 30
        "#
    )
    .bind(&doctor_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if trends.is_empty() {
        return Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "未找到该医生的评分趋势数据".to_string()
        )));
    }

    // 构建趋势数据
    let trend_data: Vec<serde_json::Value> = trends
        .into_iter()
        .map(|(_, comprehensive, cpi, influence, quality, date)| {
            serde_json::json!({
                "date": date.format("%Y-%m-%d").to_string(),
                "comprehensive_score": comprehensive,
                "cost_performance_index": cpi,
                "influence_score": influence,
                "quality_score": quality
            })
        })
        .collect();

    // 计算趋势统计
    let mut comprehensive_scores: Vec<f64> = trend_data
        .iter()
        .map(|item| item["comprehensive_score"].as_f64().unwrap_or(0.0))
        .collect();
    
    comprehensive_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let statistics = serde_json::json!({
        "data_points": trend_data.len(),
        "latest_score": trend_data.first().map(|item| item["comprehensive_score"].as_f64().unwrap_or(0.0)),
        "highest_score": comprehensive_scores.last().copied().unwrap_or(0.0),
        "lowest_score": comprehensive_scores.first().copied().unwrap_or(0.0),
        "average_score": comprehensive_scores.iter().sum::<f64>() / comprehensive_scores.len() as f64
    });

    let response = serde_json::json!({
        "doctor_id": doctor_id,
        "trends": trend_data,
        "statistics": statistics
    });

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}
