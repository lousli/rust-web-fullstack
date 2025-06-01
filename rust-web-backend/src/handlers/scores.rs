use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use crate::models::{DoctorScore, ApiResponse, QueryParams};

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
