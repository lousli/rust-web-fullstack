use actix_web::{web, HttpResponse, Result};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use csv::ReaderBuilder;
use std::collections::HashMap;
use chrono::Utc;
use crate::models::{Doctor, ApiResponse};

/// 数据导入状态
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportStatus {
    pub success: bool,
    pub message: String,
    pub total_rows: usize,
    pub success_count: usize,
    pub error_count: usize,
    pub errors: Vec<ImportError>,
}

/// 导入错误记录
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportError {
    pub row: usize,
    pub field: String,
    pub value: String,
    pub error: String,
}

/// 数据导入请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportRequest {
    pub data: String,           // CSV 数据
    pub format: String,         // 数据格式 ("csv", "json")
    pub mapping: Option<HashMap<String, String>>, // 字段映射
    pub overwrite: bool,        // 是否覆盖现有数据
}

/// 导入模板
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportTemplate {
    pub csv_headers: Vec<String>,
    pub field_descriptions: HashMap<String, String>,
    pub sample_data: Vec<HashMap<String, String>>,
    pub validation_rules: HashMap<String, String>,
}

/// 获取导入模板
pub async fn get_import_template() -> Result<HttpResponse> {
    let template = ImportTemplate {
        csv_headers: vec![
            "id".to_string(),
            "name".to_string(),
            "title".to_string(),
            "region".to_string(),
            "department".to_string(),
            "agency_name".to_string(),
            "agency_price".to_string(),
            "total_followers".to_string(),
            "total_likes".to_string(),
            "total_works".to_string(),
            "likes_7d".to_string(),
            "followers_7d".to_string(),
            "shares_7d".to_string(),
            "comments_7d".to_string(),
            "works_7d".to_string(),
            "likes_15d".to_string(),
            "followers_15d".to_string(),
            "shares_15d".to_string(),
            "comments_15d".to_string(),
            "works_15d".to_string(),
            "likes_30d".to_string(),
            "followers_30d".to_string(),
            "shares_30d".to_string(),
            "comments_30d".to_string(),
            "works_30d".to_string(),
            "performance_score".to_string(),
            "affinity_score".to_string(),
            "editing_score".to_string(),
            "video_quality_score".to_string(),
        ],
        field_descriptions: [
            ("id".to_string(), "医生唯一标识符".to_string()),
            ("name".to_string(), "医生姓名".to_string()),
            ("title".to_string(), "职称".to_string()),
            ("region".to_string(), "地区".to_string()),
            ("department".to_string(), "科室".to_string()),
            ("agency_name".to_string(), "机构名称".to_string()),
            ("agency_price".to_string(), "机构报价（元）".to_string()),
            ("total_followers".to_string(), "总粉丝量（数字）".to_string()),
            ("total_likes".to_string(), "总获赞量（数字）".to_string()),
            ("total_works".to_string(), "总作品数（数字）".to_string()),
            ("likes_7d".to_string(), "7天获赞数（数字）".to_string()),
            ("followers_7d".to_string(), "7天粉丝增量（数字）".to_string()),
            ("shares_7d".to_string(), "7天分享数（数字）".to_string()),
            ("comments_7d".to_string(), "7天评论数（数字）".to_string()),
            ("works_7d".to_string(), "7天作品数（数字）".to_string()),
            ("performance_score".to_string(), "表现力评分（0-100）".to_string()),
            ("affinity_score".to_string(), "亲和力评分（0-100）".to_string()),
            ("editing_score".to_string(), "剪辑质量评分（0-100）".to_string()),
            ("video_quality_score".to_string(), "视频质量评分（0-100）".to_string()),
        ].iter().cloned().collect(),
        sample_data: vec![
            [
                ("id".to_string(), "doc_0001".to_string()),
                ("name".to_string(), "张医生".to_string()),
                ("title".to_string(), "主任医师".to_string()),
                ("region".to_string(), "北京".to_string()),
                ("department".to_string(), "心内科".to_string()),
                ("agency_name".to_string(), "某某MCN机构".to_string()),
                ("agency_price".to_string(), "50000".to_string()),
                ("total_followers".to_string(), "1000000".to_string()),
                ("total_likes".to_string(), "500000".to_string()),
                ("total_works".to_string(), "200".to_string()),
                ("performance_score".to_string(), "85.5".to_string()),
                ("affinity_score".to_string(), "90.0".to_string()),
                ("editing_score".to_string(), "75.5".to_string()),
                ("video_quality_score".to_string(), "80.0".to_string()),
            ].iter().cloned().collect(),
        ],
        validation_rules: [
            ("name".to_string(), "必填字段，不能为空".to_string()),
            ("total_followers".to_string(), "必须是非负整数".to_string()),
            ("total_likes".to_string(), "必须是非负整数".to_string()),
            ("total_works".to_string(), "必须是非负整数".to_string()),
            ("agency_price".to_string(), "必须是非负数字".to_string()),
            ("performance_score".to_string(), "可选，范围0-100".to_string()),
            ("affinity_score".to_string(), "可选，范围0-100".to_string()),
            ("editing_score".to_string(), "可选，范围0-100".to_string()),
            ("video_quality_score".to_string(), "可选，范围0-100".to_string()),
        ].iter().cloned().collect(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(template)))
}

/// 处理CSV数据导入
pub async fn import_csv_data(
    pool: web::Data<SqlitePool>,
    request: web::Json<ImportRequest>,
) -> Result<HttpResponse> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(request.data.as_bytes());

    let headers = reader.headers()
        .map_err(|e| actix_web::error::ErrorBadRequest(format!("CSV headers error: {}", e)))?
        .clone();

    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();

    for (index, record) in reader.records().enumerate() {
        match record {
            Ok(record) => {
                match parse_doctor_from_csv(&headers, &record, index) {
                    Ok(doctor) => {
                        match save_doctor_to_db(pool.get_ref(), &doctor, request.overwrite).await {
                            Ok(_) => success_count += 1,
                            Err(e) => {
                                error_count += 1;
                                errors.push(ImportError {
                                    row: index + 1,
                                    field: "database".to_string(),
                                    value: "".to_string(),
                                    error: format!("保存失败: {}", e),
                                });
                            }
                        }
                    }
                    Err(parse_errors) => {
                        error_count += 1;
                        errors.extend(parse_errors);
                    }
                }
            }
            Err(e) => {
                error_count += 1;
                errors.push(ImportError {
                    row: index + 1,
                    field: "record".to_string(),
                    value: "".to_string(),
                    error: format!("CSV parsing error: {}", e),
                });
            }
        }
    }

    let status = ImportStatus {
        success: error_count == 0,
        message: if error_count == 0 {
            format!("成功导入{}条记录", success_count)
        } else {
            format!("导入完成，成功{}条，失败{}条", success_count, error_count)
        },
        total_rows: success_count + error_count,
        success_count,
        error_count,
        errors,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(status)))
}

/// 从CSV记录解析医生数据
fn parse_doctor_from_csv(
    headers: &csv::StringRecord,
    record: &csv::StringRecord,
    row_index: usize,
) -> Result<Doctor, Vec<ImportError>> {
    let mut errors = Vec::new();

    // 查找字段索引的辅助函数
    let find_field_index = |field_name: &str| {
        headers.iter().position(|h| h.trim().eq_ignore_ascii_case(field_name))
    };

    // 获取字段值的辅助函数
    let get_field_value = |field_name: &str| -> Option<String> {
        find_field_index(field_name)
            .and_then(|index| record.get(index))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    };

    // 解析必填字段
    let id = get_field_value("id").unwrap_or_else(|| format!("doc_{:04}", row_index + 1));
    let name = match get_field_value("name") {
        Some(name) => name,
        None => {
            errors.push(ImportError {
                row: row_index + 1,
                field: "name".to_string(),
                value: "".to_string(),
                error: "医生姓名不能为空".to_string(),
            });
            "".to_string()
        }
    };

    // 解析可选字段
    let title = get_field_value("title");
    let region = get_field_value("region");
    let department = get_field_value("department");
    let agency_name = get_field_value("agency_name");

    // 解析数字字段
    let parse_i32_field = |field_name: &str| -> Option<i32> {
        get_field_value(field_name).and_then(|value| {
            match value.parse::<i32>() {
                Ok(num) if num >= 0 => Some(num),
                _ => {
                    errors.push(ImportError {
                        row: row_index + 1,
                        field: field_name.to_string(),
                        value,
                        error: "必须是非负整数".to_string(),
                    });
                    None
                }
            }
        })
    };

    let parse_f64_field = |field_name: &str| -> Option<f64> {
        get_field_value(field_name).and_then(|value| {
            match value.parse::<f64>() {
                Ok(num) if (0.0..=100.0).contains(&num) => Some(num),
                Ok(_) => {
                    errors.push(ImportError {
                        row: row_index + 1,
                        field: field_name.to_string(),
                        value,
                        error: "必须在0-100之间".to_string(),
                    });
                    None
                }
                Err(_) => {
                    errors.push(ImportError {
                        row: row_index + 1,
                        field: field_name.to_string(),
                        value,
                        error: "必须是有效数字".to_string(),
                    });
                    None
                }
            }
        })
    };

    let agency_price = get_field_value("agency_price").and_then(|value| {
        match value.parse::<f64>() {
            Ok(price) if price >= 0.0 => Some(price),
            _ => {
                errors.push(ImportError {
                    row: row_index + 1,
                    field: "agency_price".to_string(),
                    value,
                    error: "必须是非负数字".to_string(),
                });
                None
            }
        }
    });

    let total_followers = parse_i32_field("total_followers").unwrap_or(0);
    let total_likes = parse_i32_field("total_likes").unwrap_or(0);
    let total_works = parse_i32_field("total_works").unwrap_or(0);

    // 解析时间段数据
    let likes_7d = parse_i32_field("likes_7d");
    let followers_7d = parse_i32_field("followers_7d");
    let shares_7d = parse_i32_field("shares_7d");
    let comments_7d = parse_i32_field("comments_7d");
    let works_7d = parse_i32_field("works_7d");

    let likes_15d = parse_i32_field("likes_15d");
    let followers_15d = parse_i32_field("followers_15d");
    let shares_15d = parse_i32_field("shares_15d");
    let comments_15d = parse_i32_field("comments_15d");
    let works_15d = parse_i32_field("works_15d");

    let likes_30d = parse_i32_field("likes_30d");
    let followers_30d = parse_i32_field("followers_30d");
    let shares_30d = parse_i32_field("shares_30d");
    let comments_30d = parse_i32_field("comments_30d");
    let works_30d = parse_i32_field("works_30d");

    // 解析人工评分
    let performance_score = parse_f64_field("performance_score");
    let affinity_score = parse_f64_field("affinity_score");
    let editing_score = parse_f64_field("editing_score");
    let video_quality_score = parse_f64_field("video_quality_score");

    if errors.is_empty() {
        let doctor = Doctor {
            id,
            name,
            title,
            region,
            department,
            agency_name,
            agency_price,
            total_followers,
            total_likes,
            total_works,
            likes_7d,
            followers_7d,
            shares_7d,
            comments_7d,
            works_7d,
            likes_15d,
            followers_15d,
            shares_15d,
            comments_15d,
            works_15d,
            likes_30d,
            followers_30d,
            shares_30d,
            comments_30d,
            works_30d,
            performance_score,
            affinity_score,
            editing_score,
            video_quality_score,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        Ok(doctor)
    } else {
        Err(errors)
    }
}

/// 保存医生数据到数据库
async fn save_doctor_to_db(
    pool: &SqlitePool,
    doctor: &Doctor,
    overwrite: bool,
) -> Result<(), sqlx::Error> {
    if overwrite {
        // 使用 REPLACE 语法，如果存在则替换
        sqlx::query!(
            r#"
            REPLACE INTO doctors (
                id, name, title, region, department, agency_name, agency_price,
                total_followers, total_likes, total_works,
                likes_7d, followers_7d, shares_7d, comments_7d, works_7d,
                likes_15d, followers_15d, shares_15d, comments_15d, works_15d,
                likes_30d, followers_30d, shares_30d, comments_30d, works_30d,
                performance_score, affinity_score, editing_score, video_quality_score,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            doctor.id,
            doctor.name,
            doctor.title,
            doctor.region,
            doctor.department,
            doctor.agency_name,
            doctor.agency_price,
            doctor.total_followers,
            doctor.total_likes,
            doctor.total_works,
            doctor.likes_7d,
            doctor.followers_7d,
            doctor.shares_7d,
            doctor.comments_7d,
            doctor.works_7d,
            doctor.likes_15d,
            doctor.followers_15d,
            doctor.shares_15d,
            doctor.comments_15d,
            doctor.works_15d,
            doctor.likes_30d,
            doctor.followers_30d,
            doctor.shares_30d,
            doctor.comments_30d,
            doctor.works_30d,
            doctor.performance_score,
            doctor.affinity_score,
            doctor.editing_score,
            doctor.video_quality_score,
            doctor.created_at,
            doctor.updated_at
        )
        .execute(pool)
        .await?;
    } else {
        // 使用 INSERT OR IGNORE，如果存在则忽略
        sqlx::query!(
            r#"
            INSERT OR IGNORE INTO doctors (
                id, name, title, region, department, agency_name, agency_price,
                total_followers, total_likes, total_works,
                likes_7d, followers_7d, shares_7d, comments_7d, works_7d,
                likes_15d, followers_15d, shares_15d, comments_15d, works_15d,
                likes_30d, followers_30d, shares_30d, comments_30d, works_30d,
                performance_score, affinity_score, editing_score, video_quality_score,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            doctor.id,
            doctor.name,
            doctor.title,
            doctor.region,
            doctor.department,
            doctor.agency_name,
            doctor.agency_price,
            doctor.total_followers,
            doctor.total_likes,
            doctor.total_works,
            doctor.likes_7d,
            doctor.followers_7d,
            doctor.shares_7d,
            doctor.comments_7d,
            doctor.works_7d,
            doctor.likes_15d,
            doctor.followers_15d,
            doctor.shares_15d,
            doctor.comments_15d,
            doctor.works_15d,
            doctor.likes_30d,
            doctor.followers_30d,
            doctor.shares_30d,
            doctor.comments_30d,
            doctor.works_30d,
            doctor.performance_score,
            doctor.affinity_score,
            doctor.editing_score,
            doctor.video_quality_score,
            doctor.created_at,
            doctor.updated_at
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
