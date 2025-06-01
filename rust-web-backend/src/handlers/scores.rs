use actix_web::{web, HttpResponse, Result};
use sqlx::{SqlitePool, Row};
use chrono::Utc;
use crate::models::{WeightConfig, DoctorScore, ApiResponse, AnalysisParams, Doctor, QueryParams};
use crate::algorithms::ScoringAlgorithm;
use std::collections::HashMap;

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
    }

    // 计算评分
    let scores = ScoringAlgorithm::calculate_scores(&doctors, &weight_config)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

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
        doctor_data.insert("institution_price", row.get::<i64, _>("institution_price").to_string());
        
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
    .bind(limit)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let recommendations = ScoringAlgorithm::get_investment_recommendations(&scores, &doctors, limit);

    let result: Vec<HashMap<String, serde_json::Value>> = recommendations
        .into_iter()
        .enumerate()
        .map(|(index, (doctor_id, cpi, reason))| {
            let doctor = doctors.iter().find(|d| d.id == doctor_id);
            let mut map = HashMap::new();
            
            map.insert("rank".to_string(), serde_json::Value::Number(serde_json::Number::from(index + 1)));
            map.insert("doctor_id".to_string(), serde_json::Value::String(doctor_id));
            map.insert("cost_performance_index".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(cpi).unwrap()));
            map.insert("reason".to_string(), serde_json::Value::String(reason));
            
            if let Some(d) = doctor {
                map.insert("name".to_string(), serde_json::Value::String(d.name.clone()));
                map.insert("title".to_string(), serde_json::Value::String(d.title.clone()));
                map.insert("department".to_string(), serde_json::Value::String(d.department.clone()));
                map.insert("institution_price".to_string(), serde_json::Value::Number(serde_json::Number::from(d.institution_price)));
            }
            
            map
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

/// 获取趋势分析数据
pub async fn get_trends(
    pool: web::Data<SqlitePool>,
    params: web::Query<AnalysisParams>,
) -> Result<HttpResponse> {
    // 这里可以实现基于时间的趋势分析
    // 目前返回一个模拟的趋势数据
    let mut trends = HashMap::new();
    
    // 粉丝增长趋势
    let fan_growth: Vec<(String, i64)> = sqlx::query_as(
        "SELECT department, AVG(fans_15d) as avg_growth FROM doctors GROUP BY department ORDER BY avg_growth DESC"
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
            id: 1,
            doctor_id: 1,
            doctor_name: "张伟明".to_string(),
            department: "心内科".to_string(),
            region: "北京".to_string(),
            institution: "北京协和医院".to_string(),
            account_type: "头部".to_string(),
            cost_performance_score: 92.5,
            data_index_score: 89.3,
            performance_score: 94.8,
            affinity_score: 91.2,
            editing_score: 88.7,
            video_quality_score: 93.1,
            weighted_total_score: 91.6,
            calculated_at: Utc::now(),
        },
        DoctorScore {
            id: 2,
            doctor_id: 2,
            doctor_name: "李小红".to_string(),
            department: "神经内科".to_string(),
            region: "上海".to_string(),
            institution: "上海华山医院".to_string(),
            account_type: "腰部".to_string(),
            cost_performance_score: 85.2,
            data_index_score: 87.6,
            performance_score: 82.4,
            affinity_score: 89.1,
            editing_score: 84.3,
            video_quality_score: 86.8,
            weighted_total_score: 85.9,
            calculated_at: Utc::now(),
        },
        DoctorScore {
            id: 3,
            doctor_id: 3,
            doctor_name: "王建国".to_string(),
            department: "骨科".to_string(),
            region: "广州".to_string(),
            institution: "中山大学附属第一医院".to_string(),
            account_type: "尾部".to_string(),
            cost_performance_score: 78.9,
            data_index_score: 76.4,
            performance_score: 79.2,
            affinity_score: 77.8,
            editing_score: 75.6,
            video_quality_score: 78.1,
            weighted_total_score: 77.7,
            calculated_at: Utc::now(),
        },
        DoctorScore {
            id: 4,
            doctor_id: 4,
            doctor_name: "陈雅文".to_string(),
            department: "妇产科".to_string(),
            region: "深圳".to_string(),
            institution: "深圳市人民医院".to_string(),
            account_type: "头部".to_string(),
            cost_performance_score: 95.1,
            data_index_score: 92.8,
            performance_score: 96.3,
            affinity_score: 94.7,
            editing_score: 91.5,
            video_quality_score: 95.9,
            weighted_total_score: 94.4,
            calculated_at: Utc::now(),
        },
        DoctorScore {
            id: 5,
            doctor_id: 5,
            doctor_name: "刘志强".to_string(),
            department: "儿科".to_string(),
            region: "杭州".to_string(),
            institution: "浙江大学医学院附属儿童医院".to_string(),
            account_type: "腰部".to_string(),
            cost_performance_score: 88.3,
            data_index_score: 85.7,
            performance_score: 87.9,
            affinity_score: 90.4,
            editing_score: 86.2,
            video_quality_score: 88.8,
            weighted_total_score: 87.9,
            calculated_at: Utc::now(),
        },
        DoctorScore {
            id: 6,
            doctor_id: 6,
            doctor_name: "赵美丽".to_string(),
            department: "眼科".to_string(),
            region: "成都".to_string(),
            institution: "四川大学华西医院".to_string(),
            account_type: "尾部".to_string(),
            cost_performance_score: 72.6,
            data_index_score: 74.1,
            performance_score: 71.8,
            affinity_score: 75.3,
            editing_score: 73.7,
            video_quality_score: 72.9,
            weighted_total_score: 73.4,
            calculated_at: Utc::now(),
        },
        DoctorScore {
            id: 7,
            doctor_id: 7,
            doctor_name: "孙立华".to_string(),
            department: "呼吸内科".to_string(),
            region: "西安".to_string(),
            institution: "西安交通大学第一附属医院".to_string(),
            account_type: "头部".to_string(),
            cost_performance_score: 90.7,
            data_index_score: 88.9,
            performance_score: 92.1,
            affinity_score: 89.6,
            editing_score: 87.4,
            video_quality_score: 91.3,
            weighted_total_score: 90.0,
            calculated_at: Utc::now(),
        },
        DoctorScore {
            id: 8,
            doctor_id: 8,
            doctor_name: "周晓明".to_string(),
            department: "消化内科".to_string(),
            region: "武汉".to_string(),
            institution: "华中科技大学同济医学院附属同济医院".to_string(),
            account_type: "腰部".to_string(),
            cost_performance_score: 83.4,
            data_index_score: 81.7,
            performance_score: 84.2,
            affinity_score: 82.9,
            editing_score: 80.8,
            video_quality_score: 83.6,
            weighted_total_score: 82.8,
            calculated_at: Utc::now(),
        },
    ]
}

/// 获取医生评分列表
pub async fn get_scores(query: web::Query<QueryParams>) -> Result<HttpResponse> {
    let mut scores = get_mock_doctor_scores();
    
    // 应用筛选条件
    if let Some(ref name) = query.name {
        scores.retain(|s| s.doctor_name.contains(name));
    }
    
    if let Some(ref department) = query.department {
        scores.retain(|s| s.department == *department);
    }
    
    if let Some(ref region) = query.region {
        scores.retain(|s| s.region == *region);
    }
    
    if let Some(ref account_type) = query.account_type {
        scores.retain(|s| s.account_type == *account_type);
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
            }
            "name" => {
                scores.sort_by(|a, b| {
                    if ascending {
                        a.doctor_name.cmp(&b.doctor_name)
                    } else {
                        b.doctor_name.cmp(&a.doctor_name)
                    }
                });
            }
            _ => {}
        }
    } else {
        // 默认按总分降序排列
        scores.sort_by(|a, b| b.weighted_total_score.partial_cmp(&a.weighted_total_score).unwrap());
    }
    
    let total = scores.len() as u32;
    
    // 分页
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(scores.len());
    
    let paginated_scores = if start < scores.len() {
        scores[start..end].to_vec()
    } else {
        vec![]
    };
    
    Ok(HttpResponse::Ok().json(ApiResponse::success_with_total(paginated_scores, total)))
}

/// 获取单个医生的评分详情
pub async fn get_doctor_score(path: web::Path<u32>) -> Result<HttpResponse> {
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
        let entry = dept_stats.entry(score.department.clone()).or_insert(vec![]);
        entry.push(score.weighted_total_score);
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
