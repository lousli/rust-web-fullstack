use actix_web::{web, HttpResponse, Result};
use sqlx::{SqlitePool, Row};
use serde_json::json;
use crate::models::{ApiResponse, Doctor, MedicalWeightConfig};

/// 批量重新计算医生医疗评分
pub async fn recalculate_medical_scores(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse> {    // 获取默认的医疗权重配置
    let weight_config: Option<MedicalWeightConfig> = sqlx::query_as(
        "SELECT * FROM medical_weight_configs WHERE is_default = 1 LIMIT 1"
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let weight_config = match weight_config {
        Some(config) => config,
        None => return Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "权重配置不存在".to_string()
        ))),
    };

    // 获取所有医生数据
    let doctors: Vec<Doctor> = sqlx::query_as(
        "SELECT * FROM doctors"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut updated_count = 0;
    let mut errors = Vec::new();

    // 开始事务
    let mut tx = pool.begin()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    for doctor in &doctors {
        match recalculate_doctor_score(&mut tx, doctor, &weight_config).await {
            Ok(_) => updated_count += 1,
            Err(e) => errors.push(format!("医生 {} 评分计算失败: {}", doctor.name, e)),
        }
    }

    if errors.is_empty() {
        // 提交事务
        tx.commit()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;        Ok(HttpResponse::Ok().json(ApiResponse::success(json!({
            "updated_count": updated_count,
            "total_doctors": doctors.len(),
            "weight_config_id": weight_config.id,
            "weight_config_name": weight_config.name,
            "message": format!("成功重新计算 {} 位医生的评分", updated_count)
        }))))
    } else {
        // 回滚事务
        tx.rollback()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("批量计算失败，错误详情: {:?}", errors)
        )))
    }
}

/// 使用医疗权重配置重新计算单个医生评分
async fn recalculate_doctor_score(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    doctor: &Doctor,
    weight_config: &MedicalWeightConfig,
) -> Result<(), sqlx::Error> {
    // 使用新的五大核心指标权重计算评分
    let scores = calculate_medical_comprehensive_score(doctor, weight_config);

    // 更新或插入计算指标记录
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO medical_calculated_indicators (
            doctor_id, weight_config_id,
            account_influence_score, cost_effectiveness_score, content_quality_score,
            medical_credibility_score, roi_prediction_score, comprehensive_score,
            calculated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
        "#,
    )
    .bind(&doctor.id)
    .bind(weight_config.id)
    .bind(scores.account_influence_score)
    .bind(scores.cost_effectiveness_score)
    .bind(scores.content_quality_score)
    .bind(scores.medical_credibility_score)
    .bind(scores.roi_prediction_score)
    .bind(scores.comprehensive_score)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// 医疗评分结构
#[derive(Debug)]
struct MedicalScores {
    pub account_influence_score: f64,
    pub cost_effectiveness_score: f64,
    pub content_quality_score: f64,
    pub medical_credibility_score: f64,
    pub roi_prediction_score: f64,
    pub comprehensive_score: f64,
}

/// 计算医疗综合评分
fn calculate_medical_comprehensive_score(
    doctor: &Doctor,
    weight_config: &MedicalWeightConfig,
) -> MedicalScores {
    // 1. 账号影响力评分 (基于粉丝数量和互动数据)
    let account_influence_score = calculate_account_influence_score(doctor);
    
    // 2. 性价比评分 (基于价格和效果)
    let cost_effectiveness_score = calculate_cost_effectiveness_score(doctor);
    
    // 3. 内容质量评分 (基于互动率和内容表现)
    let content_quality_score = calculate_content_quality_score(doctor);
    
    // 4. 医疗可信度评分 (基于职称和专业背景)
    let medical_credibility_score = calculate_medical_credibility_score(doctor);
    
    // 5. ROI预测评分 (基于历史表现和趋势)
    let roi_prediction_score = calculate_roi_prediction_score(doctor);

    // 计算加权综合评分
    let comprehensive_score = 
        (account_influence_score * weight_config.account_influence_weight as f64 / 100.0) +
        (cost_effectiveness_score * weight_config.cost_effectiveness_weight as f64 / 100.0) +
        (content_quality_score * weight_config.content_quality_weight as f64 / 100.0) +
        (medical_credibility_score * weight_config.medical_credibility_weight as f64 / 100.0) +
        (roi_prediction_score * weight_config.roi_prediction_weight as f64 / 100.0);

    MedicalScores {
        account_influence_score,
        cost_effectiveness_score,
        content_quality_score,
        medical_credibility_score,
        roi_prediction_score,
        comprehensive_score,
    }
}

/// 计算账号影响力评分
fn calculate_account_influence_score(doctor: &Doctor) -> f64 {
    let followers_score = if doctor.total_followers >= 1000000 { 100.0 }
        else if doctor.total_followers >= 500000 { 85.0 }
        else if doctor.total_followers >= 100000 { 70.0 }
        else if doctor.total_followers >= 50000 { 55.0 }
        else { 40.0 };

    let engagement_rate = if doctor.total_works > 0 {
        (doctor.total_likes as f64 / doctor.total_works as f64) / doctor.total_followers as f64 * 100.0
    } else {
        0.0
    };

    let engagement_score = if engagement_rate >= 5.0 { 100.0 }
        else if engagement_rate >= 3.0 { 80.0 }
        else if engagement_rate >= 1.0 { 60.0 }
        else { 30.0 };    let final_score: f64 = followers_score * 0.7 + engagement_score * 0.3;
    final_score.min(100.0)
}

/// 计算性价比评分
fn calculate_cost_effectiveness_score(doctor: &Doctor) -> f64 {
    // 处理Option类型的agency_price
    let price = doctor.agency_price.unwrap_or(0.0);
    if price <= 0.0 || doctor.total_followers <= 0 {
        return 0.0;
    }
    
    let price_per_follower = price / doctor.total_followers as f64 * 1000.0;
    
    let price_score = if price_per_follower <= 0.05 { 100.0 }
        else if price_per_follower <= 0.1 { 85.0 }
        else if price_per_follower <= 0.2 { 70.0 }
        else if price_per_follower <= 0.5 { 55.0 }
        else { 30.0 };

    price_score
}

/// 计算内容质量评分
fn calculate_content_quality_score(doctor: &Doctor) -> f64 {
    if doctor.total_works == 0 {
        return 0.0;
    }

    let avg_likes_per_work = doctor.total_likes as f64 / doctor.total_works as f64;
    let like_score = if avg_likes_per_work >= 10000.0 { 100.0 }
        else if avg_likes_per_work >= 5000.0 { 80.0 }
        else if avg_likes_per_work >= 1000.0 { 60.0 }
        else { 40.0 };

    like_score
}

/// 计算医疗可信度评分
fn calculate_medical_credibility_score(doctor: &Doctor) -> f64 {
    let title_score = match doctor.title.as_deref() {
        Some("主任医师") => 100.0,
        Some("副主任医师") => 85.0,
        Some("主治医师") => 70.0,
        Some("住院医师") => 55.0,
        _ => 40.0,
    };

    // 基于科室的专业度评分
    let department_score = match doctor.department.as_deref() {
        Some("心血管内科") | Some("神经内科") | Some("消化内科") | Some("呼吸内科") => 95.0,
        Some("内分泌科") | Some("肾内科") | Some("血液科") => 90.0,
        Some("骨科") | Some("外科") | Some("妇产科") => 85.0,
        _ => 75.0,
    };    let final_score: f64 = title_score * 0.6 + department_score * 0.4;
    final_score.min(100.0)
}

/// 计算ROI预测评分
fn calculate_roi_prediction_score(doctor: &Doctor) -> f64 {
    // 基于7天数据趋势
    let growth_rate_7d = if let Some(followers_7d) = doctor.followers_7d {
        if followers_7d > 0 && doctor.total_followers > 0 {
            followers_7d as f64 / doctor.total_followers as f64 * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    };

    let trend_score = if growth_rate_7d >= 2.0 { 100.0 }
        else if growth_rate_7d >= 1.0 { 80.0 }
        else if growth_rate_7d >= 0.5 { 60.0 }
        else { 40.0 };

    trend_score
}

/// 获取医生的医疗评分详情
pub async fn get_medical_score_details(
    pool: web::Data<SqlitePool>,
    doctor_id: web::Path<String>,
) -> Result<HttpResponse> {
    let doctor_id = doctor_id.into_inner();

    // 获取医生信息
    let doctor: Option<Doctor> = sqlx::query_as(
        "SELECT * FROM doctors WHERE id = ?"
    )
    .bind(&doctor_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let doctor = match doctor {
        Some(d) => d,
        None => return Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "医生不存在".to_string()
        ))),
    };

    // 获取最新的医疗评分记录
    let score_record = sqlx::query(
        r#"
        SELECT 
            mci.*,
            mwc.name as weight_config_name,
            mwc.strategy_type
        FROM medical_calculated_indicators mci
        LEFT JOIN medical_weight_configs mwc ON mci.weight_config_id = mwc.id
        WHERE mci.doctor_id = ?
        ORDER BY mci.calculated_at DESC
        LIMIT 1
        "#,
    )
    .bind(&doctor_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let response = if let Some(record) = score_record {
        json!({
            "doctor": doctor,
            "medical_scores": {
                "account_influence_score": record.get::<f64, _>("account_influence_score"),
                "cost_effectiveness_score": record.get::<f64, _>("cost_effectiveness_score"),
                "content_quality_score": record.get::<f64, _>("content_quality_score"),
                "medical_credibility_score": record.get::<f64, _>("medical_credibility_score"),
                "roi_prediction_score": record.get::<f64, _>("roi_prediction_score"),
                "comprehensive_score": record.get::<f64, _>("comprehensive_score"),
                "calculated_at": record.get::<String, _>("calculated_at"),
                "weight_config_name": record.get::<Option<String>, _>("weight_config_name"),
                "strategy_type": record.get::<Option<String>, _>("strategy_type")
            }
        })
    } else {
        json!({
            "doctor": doctor,
            "medical_scores": null,
            "message": "尚未计算医疗评分"
        })
    };    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// 获取单个医生的医疗评分
pub async fn get_doctor_medical_score(
    pool: web::Data<SqlitePool>,
    doctor_id: web::Path<String>,
) -> Result<HttpResponse> {
    let doctor_id = doctor_id.into_inner();

    // 获取医生信息
    let doctor: Option<Doctor> = sqlx::query_as(
        "SELECT * FROM doctors WHERE id = ?"
    )
    .bind(&doctor_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let doctor = match doctor {
        Some(d) => d,
        None => return Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "医生不存在".to_string()
        ))),
    };

    // 获取最新的医疗评分记录
    let score_record = sqlx::query(
        r#"
        SELECT 
            account_influence_score, cost_effectiveness_score, content_quality_score,
            medical_credibility_score, roi_prediction_score, comprehensive_score,
            calculated_at
        FROM medical_calculated_indicators
        WHERE doctor_id = ?
        ORDER BY calculated_at DESC
        LIMIT 1
        "#,
    )
    .bind(&doctor_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let response = if let Some(record) = score_record {
        json!({
            "doctor_id": doctor_id,
            "doctor_name": doctor.name,
            "medical_scores": {
                "account_influence_score": record.get::<f64, _>("account_influence_score"),
                "cost_effectiveness_score": record.get::<f64, _>("cost_effectiveness_score"),
                "content_quality_score": record.get::<f64, _>("content_quality_score"),
                "medical_credibility_score": record.get::<f64, _>("medical_credibility_score"),
                "roi_prediction_score": record.get::<f64, _>("roi_prediction_score"),
                "comprehensive_score": record.get::<f64, _>("comprehensive_score"),
                "calculated_at": record.get::<String, _>("calculated_at")
            }
        })
    } else {
        json!({
            "doctor_id": doctor_id,
            "doctor_name": doctor.name,
            "medical_scores": null,
            "message": "尚未计算医疗评分"
        })
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}
