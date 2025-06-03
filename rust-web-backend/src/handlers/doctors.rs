use actix_web::{web, HttpResponse, Result};
use sqlx::{SqlitePool, Row};
use chrono::Utc;
use crate::models::{
    Doctor, DoctorQueryParams, DoctorListResponse, DoctorSummaryDto, 
    PaginationInfo, ApiResponse, DoctorDetailDto, DoctorImport
};

/// 获取医生列表
pub async fn get_doctors(
    pool: web::Data<SqlitePool>,
    query: web::Query<DoctorQueryParams>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;    // 构建查询条件
    let mut where_conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(ref region) = query.region {
        where_conditions.push("region = ?");
        params.push(region.clone());
    }
    if let Some(ref department) = query.department {
        where_conditions.push("department = ?");
        params.push(department.clone());
    }
    if let Some(ref search) = query.search {
        where_conditions.push("(name LIKE ? OR title LIKE ? OR agency_name LIKE ?)");
        let search_pattern = format!("%{}%", search);
        params.push(search_pattern.clone());
        params.push(search_pattern.clone());
        params.push(search_pattern);
    }

    let where_clause = if where_conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_conditions.join(" AND "))
    };

    // 排序
    let sort_by = query.sort_by.as_deref().unwrap_or("name");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");
    let order_clause = format!("ORDER BY {} {}", sort_by, sort_order);

    // 查询总数
    let count_sql = format!("SELECT COUNT(*) FROM doctors {}", where_clause);
    let mut count_query = sqlx::query(&count_sql);
    for param in &params {
        count_query = count_query.bind(param);
    }
    let total_count: i64 = count_query
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .get(0);

    // 查询数据
    let data_sql = format!(
        "SELECT * FROM doctors {} {} LIMIT ? OFFSET ?",
        where_clause, order_clause
    );
    let mut data_query = sqlx::query_as::<_, Doctor>(&data_sql);
    for param in &params {
        data_query = data_query.bind(param);
    }
    data_query = data_query.bind(page_size).bind(offset);

    let doctors = data_query
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 转换为 DTO
    let doctor_summaries: Vec<DoctorSummaryDto> = doctors
        .into_iter()
        .map(|d| DoctorSummaryDto {
            id: d.id,
            name: d.name,
            title: d.title,
            region: d.region,
            department: d.department,
            agency_price: d.agency_price,
            total_followers: d.total_followers,
            account_type: None, // 需要从评分数据中获取
            comprehensive_index: None, // 需要从评分数据中获取
            cost_effectiveness_index: None, // 需要从评分数据中获取
            rank: None, // 需要从排名数据中获取
        })
        .collect();

    let pagination = PaginationInfo {
        page,
        page_size,
        total_count,
        total_pages: (total_count as f64 / page_size as f64).ceil() as i32,
    };

    let response = DoctorListResponse {
        doctors: doctor_summaries,
        pagination,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// 创建新医生
pub async fn create_doctor(
    pool: web::Data<SqlitePool>,
    doctor: web::Json<Doctor>,
) -> Result<HttpResponse> {
    let doctor = doctor.into_inner();

    let result = sqlx::query(
        r#"
        INSERT INTO doctors (
            id, name, title, region, department, agency_name, agency_price,
            total_followers, total_likes, total_works
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&doctor.id)
    .bind(&doctor.name)
    .bind(&doctor.title)
    .bind(&doctor.region)
    .bind(&doctor.department)
    .bind(&doctor.agency_name)
    .bind(doctor.agency_price)
    .bind(doctor.total_followers)
    .bind(doctor.total_likes)
    .bind(doctor.total_works)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(doctor))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("创建医生失败: {}", e)
        ))),
    }
}

/// 获取单个医生详情
pub async fn get_doctor(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let doctor_id = path.into_inner();

    // 获取医生基础信息
    let doctor: Option<Doctor> = sqlx::query_as(
        "SELECT * FROM doctors WHERE id = ?"
    )
    .bind(&doctor_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if let Some(doctor) = doctor {
        // 这里可以添加获取关联数据的逻辑（指标、评分、计算结果等）
        let detail = DoctorDetailDto {
            doctor,
            metrics: None,
            scores: None,
            indicators: None,
        };

        Ok(HttpResponse::Ok().json(ApiResponse::success(detail)))
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "医生不存在".to_string()
        )))
    }
}

/// 更新医生信息
pub async fn update_doctor(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
    doctor: web::Json<Doctor>,
) -> Result<HttpResponse> {
    let doctor_id = path.into_inner();
    let doctor = doctor.into_inner();

    let result = sqlx::query(
        r#"
        UPDATE doctors SET
            name = ?, title = ?, region = ?, department = ?, 
            agency_name = ?, agency_price = ?, total_followers = ?,
            total_likes = ?, total_works = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&doctor.name)
    .bind(&doctor.title)
    .bind(&doctor.region)
    .bind(&doctor.department)
    .bind(&doctor.agency_name)
    .bind(doctor.agency_price)
    .bind(doctor.total_followers)
    .bind(doctor.total_likes)
    .bind(doctor.total_works)
    .bind(Utc::now())
    .bind(&doctor_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(doctor)))
        }
        Ok(_) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "医生不存在".to_string()
        ))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("更新医生失败: {}", e)
        ))),
    }
}

/// 删除医生
pub async fn delete_doctor(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let doctor_id = path.into_inner();

    let result = sqlx::query("DELETE FROM doctors WHERE id = ?")
        .bind(&doctor_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => {
            Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(())))
        }
        Ok(_) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "医生不存在".to_string()
        ))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("删除医生失败: {}", e)
        ))),
    }
}

/// 导入CSV数据
pub async fn import_csv(
    pool: web::Data<SqlitePool>,
    data: web::Json<Vec<DoctorImport>>,
) -> Result<HttpResponse> {
    let import_data = data.into_inner();
    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();

    for (index, import_doctor) in import_data.iter().enumerate() {
        // 生成医生ID
        let doctor_id = format!("doc_{:04}", index + 1);        let doctor = Doctor {
            id: doctor_id,
            name: import_doctor.name.clone(),
            title: import_doctor.title.clone(),
            region: import_doctor.region.clone(),
            department: import_doctor.department.clone(),
            agency_name: import_doctor.agency_name.clone(),
            agency_price: import_doctor.agency_price,
            total_followers: import_doctor.total_followers.unwrap_or(0),
            total_likes: import_doctor.total_likes.unwrap_or(0),
            total_works: import_doctor.total_works.unwrap_or(0),
            
            // 7天数据
            likes_7d: None,
            followers_7d: None,
            shares_7d: None,
            comments_7d: None,
            works_7d: None,
            
            // 15天数据
            likes_15d: None,
            followers_15d: None,
            shares_15d: None,
            comments_15d: None,
            works_15d: None,
            
            // 30天数据
            likes_30d: None,
            followers_30d: None,
            shares_30d: None,
            comments_30d: None,
            works_30d: None,
            
            // 人工评分
            performance_score: None,
            affinity_score: None,
            editing_score: None,
            video_quality_score: None,
            
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        // 插入医生基础数据
        let result = sqlx::query(
            r#"
            INSERT OR REPLACE INTO doctors (
                id, name, title, region, department, agency_name, agency_price,
                total_followers, total_likes, total_works, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&doctor.id)
        .bind(&doctor.name)
        .bind(&doctor.title)
        .bind(&doctor.region)
        .bind(&doctor.department)
        .bind(&doctor.agency_name)
        .bind(doctor.agency_price)
        .bind(doctor.total_followers)
        .bind(doctor.total_likes)
        .bind(doctor.total_works)
        .bind(doctor.created_at)
        .bind(doctor.updated_at)
        .execute(pool.get_ref())
        .await;

        match result {
            Ok(_) => success_count += 1,
            Err(e) => {
                error_count += 1;
                errors.push(format!("第{}行导入失败: {}", index + 1, e));
            }
        }
    }

    let response = serde_json::json!({
        "success": error_count == 0,
        "message": if error_count == 0 { "导入成功" } else { "部分导入失败" },
        "total_count": import_data.len(),
        "success_count": success_count,
        "error_count": error_count,
        "errors": errors
    });

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// 导出CSV数据
pub async fn export_csv(
    pool: web::Data<SqlitePool>,
    _query: web::Query<DoctorQueryParams>,
) -> Result<HttpResponse> {
    // 获取所有医生数据
    let doctors: Vec<Doctor> = sqlx::query_as("SELECT * FROM doctors ORDER BY name")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 生成CSV格式的响应
    let mut csv_content = String::from("姓名,职称,地区,科室,机构名称,机构报价,总粉丝量,总获赞量,总作品数\n");
    
    for doctor in doctors {
        csv_content.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            doctor.name,
            doctor.title.unwrap_or_default(),
            doctor.region.unwrap_or_default(),
            doctor.department.unwrap_or_default(),
            doctor.agency_name.unwrap_or_default(),
            doctor.agency_price.unwrap_or(0.0),
            doctor.total_followers,
            doctor.total_likes,
            doctor.total_works
        ));
    }

    Ok(HttpResponse::Ok()
        .content_type("text/csv; charset=utf-8")
        .insert_header(("Content-Disposition", "attachment; filename=\"doctors.csv\""))
        .body(csv_content))
}

/// 获取医生统计数据
pub async fn get_statistics(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    // 总医生数
    let total_doctors: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM doctors")
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 按地区统计
    let region_stats: Vec<(String, i64)> = sqlx::query_as(
        "SELECT region, COUNT(*) FROM doctors WHERE region IS NOT NULL GROUP BY region ORDER BY COUNT(*) DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 按科室统计
    let department_stats: Vec<(String, i64)> = sqlx::query_as(
        "SELECT department, COUNT(*) FROM doctors WHERE department IS NOT NULL GROUP BY department ORDER BY COUNT(*) DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 平均粉丝数
    let avg_followers: f64 = sqlx::query_scalar("SELECT AVG(total_followers) FROM doctors")
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let statistics = serde_json::json!({
        "total_doctors": total_doctors,
        "average_followers": avg_followers,
        "region_distribution": region_stats,
        "department_distribution": department_stats
    });

    Ok(HttpResponse::Ok().json(ApiResponse::success(statistics)))
}