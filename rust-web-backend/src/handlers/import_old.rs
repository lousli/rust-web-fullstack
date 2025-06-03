use actix_web::{web, HttpResponse, Result};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use csv::ReaderBuilder;
use std::collections::HashMap;
use crate::models::Doctor;

/// 解析数字字段的宏
macro_rules! parse_i64_field {
    ($field_name:expr, $doctor_field:expr) => {
        if let Some(index) = find_field_index($field_name) {
            if let Some(value) = record.get(index) {
                if !value.trim().is_empty() {
                    match value.trim().parse::<i32>() {
                        Ok(num) => $doctor_field = num,
                        Err(_) => {
                            errors.push(ImportError {
                                row: row_index + 1,
                                field: $field_name.to_string(),
                                value: value.to_string(),
                                error: "无效的数字格式".to_string(),
                            });
                        }
                    }
                }
            }
        }
    };
}

macro_rules! parse_f32_field {
    ($field_name:expr, $doctor_field:expr) => {
        if let Some(index) = find_field_index($field_name) {
            if let Some(value) = record.get(index) {
                if !value.trim().is_empty() {
                    match value.trim().parse::<f64>() {
                        Ok(num) => $doctor_field = Some(num),
                        Err(_) => {
                            errors.push(ImportError {
                                row: row_index + 1,
                                field: $field_name.to_string(),
                                value: value.to_string(),
                                error: "无效的浮点数格式".to_string(),
                            });
                        }
                    }
                }
            }
        }
    };
}

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

/// 获取导入模板
pub async fn get_import_template() -> Result<HttpResponse> {
    let template = ImportTemplate {
        csv_headers: vec![
            "id".to_string(),
            "name".to_string(),
            "title".to_string(),
            "region".to_string(),
            "department".to_string(),
            "institution".to_string(),
            "institution_price".to_string(),
            "list_price".to_string(),
            "total_fans".to_string(),
            "total_likes".to_string(),
            "total_works".to_string(),
            "likes_7d".to_string(),
            "fans_15d".to_string(),
            "shares_30d".to_string(),
            "comments_30d".to_string(),
            "works_7d".to_string(),
            "performance_score".to_string(),
            "editing_score".to_string(),
            "visual_score".to_string(),
        ],
        sample_data: vec![
            "DOC001,张医生,主任医师,北京,内科,北京医院,50000,60000,10000,5000,100,500,200,300,150,10,85.5,78.2,90.1".to_string(),
            "DOC002,李医生,副主任医师,上海,外科,上海医院,45000,55000,8000,4500,80,400,180,250,120,8,82.0,75.5,88.3".to_string(),
        ],
        field_descriptions: HashMap::from([
            ("id".to_string(), "医生ID（必填，唯一标识）".to_string()),
            ("name".to_string(), "医生姓名（必填）".to_string()),
            ("title".to_string(), "职称（必填）".to_string()),
            ("region".to_string(), "地区（必填）".to_string()),
            ("department".to_string(), "科室（必填）".to_string()),
            ("institution".to_string(), "机构名称（可选）".to_string()),
            ("institution_price".to_string(), "机构报价（分，数字）".to_string()),
            ("list_price".to_string(), "刊例价（分，数字，可选）".to_string()),
            ("total_fans".to_string(), "总粉丝量（数字）".to_string()),
            ("total_likes".to_string(), "总获赞量（数字）".to_string()),
            ("total_works".to_string(), "总作品数（数字）".to_string()),
            ("likes_7d".to_string(), "7天新增点赞（数字）".to_string()),
            ("fans_15d".to_string(), "15天净增粉丝（数字）".to_string()),
            ("shares_30d".to_string(), "30天新增分享（数字）".to_string()),
            ("comments_30d".to_string(), "30天新增评论（数字）".to_string()),
            ("works_7d".to_string(), "7天新增作品（数字）".to_string()),
            ("performance_score".to_string(), "表现力评分（0-100小数）".to_string()),
            ("editing_score".to_string(), "剪辑水平评分（0-100小数）".to_string()),
            ("visual_score".to_string(), "画面质量评分（0-100小数）".to_string()),
        ]),
    };

    Ok(HttpResponse::Ok().json(template))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportTemplate {
    pub csv_headers: Vec<String>,
    pub sample_data: Vec<String>,
    pub field_descriptions: HashMap<String, String>,
}

/// 导入医生数据
pub async fn import_doctors(
    pool: web::Data<SqlitePool>,
    import_req: web::Json<ImportRequest>,
) -> Result<HttpResponse> {
    let mut status = ImportStatus {
        success: true,
        message: "数据导入成功".to_string(),
        total_rows: 0,
        success_count: 0,
        error_count: 0,
        errors: Vec::new(),
    };

    match import_req.format.as_str() {
        "csv" => {
            status = import_csv_data(&pool, &import_req.data, &import_req.mapping, import_req.overwrite).await;
        },
        "json" => {
            status = import_json_data(&pool, &import_req.data, import_req.overwrite).await;
        },
        _ => {
            status.success = false;
            status.message = "不支持的数据格式".to_string();
        }
    }

    let response_status = if status.success { 200 } else { 400 };
    Ok(HttpResponse::build(actix_web::http::StatusCode::from_u16(response_status).unwrap()).json(status))
}

/// 导入 CSV 数据
async fn import_csv_data(
    pool: &SqlitePool,
    csv_data: &str,
    mapping: &Option<HashMap<String, String>>,
    overwrite: bool,
) -> ImportStatus {
    let mut status = ImportStatus {
        success: true,
        message: "CSV 数据导入成功".to_string(),
        total_rows: 0,
        success_count: 0,
        error_count: 0,
        errors: Vec::new(),
    };

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());

    let headers = match reader.headers() {
        Ok(headers) => headers.clone(),
        Err(e) => {
            status.success = false;
            status.message = format!("CSV 格式错误: {}", e);
            return status;
        }
    };

    for (row_index, result) in reader.records().enumerate() {
        status.total_rows += 1;
        
        match result {
            Ok(record) => {
                match parse_csv_record(&headers, &record, mapping, row_index + 2) {
                    Ok(doctor) => {
                        if let Err(e) = save_doctor(pool, &doctor, overwrite).await {
                            status.error_count += 1;
                            status.errors.push(ImportError {
                                row: row_index + 2,
                                field: "database".to_string(),
                                value: "".to_string(),
                                error: format!("数据库保存失败: {}", e),
                            });
                        } else {
                            status.success_count += 1;
                        }
                    },
                    Err(errors) => {
                        status.error_count += 1;
                        status.errors.extend(errors);
                    }
                }
            },
            Err(e) => {
                status.error_count += 1;
                status.errors.push(ImportError {
                    row: row_index + 2,
                    field: "csv".to_string(),
                    value: "".to_string(),
                    error: format!("CSV 行解析错误: {}", e),
                });
            }
        }
    }

    if status.error_count > 0 {
        status.success = false;
        status.message = format!("部分数据导入失败，成功: {}, 失败: {}", status.success_count, status.error_count);
    }

    status
}

/// 导入 JSON 数据
async fn import_json_data(
    pool: &SqlitePool,
    json_data: &str,
    overwrite: bool,
) -> ImportStatus {
    let mut status = ImportStatus {
        success: true,
        message: "JSON 数据导入成功".to_string(),
        total_rows: 0,
        success_count: 0,
        error_count: 0,
        errors: Vec::new(),
    };

    let doctors: Result<Vec<Doctor>, _> = serde_json::from_str(json_data);
    
    match doctors {
        Ok(doctors) => {
            status.total_rows = doctors.len();
            
            for (index, doctor) in doctors.iter().enumerate() {
                if let Err(e) = save_doctor(pool, doctor, overwrite).await {
                    status.error_count += 1;
                    status.errors.push(ImportError {
                        row: index + 1,
                        field: "database".to_string(),
                        value: doctor.id.clone(),
                        error: format!("数据库保存失败: {}", e),
                    });
                } else {
                    status.success_count += 1;
                }
            }
        },
        Err(e) => {
            status.success = false;
            status.message = format!("JSON 格式错误: {}", e);
        }
    }

    if status.error_count > 0 && status.success_count > 0 {
        status.success = false;
        status.message = format!("部分数据导入失败，成功: {}, 失败: {}", status.success_count, status.error_count);
    }

    status
}

/// 解析 CSV 记录
fn parse_csv_record(
    headers: &csv::StringRecord,
    record: &csv::StringRecord,
    mapping: &Option<HashMap<String, String>>,
    row_number: usize,
) -> Result<Doctor, Vec<ImportError>> {
    let mut errors = Vec::new();    let mut doctor = Doctor {
        id: String::new(),
        name: String::new(),
        title: None,
        region: None,
        department: None,
        agency_name: None,
        agency_price: None,
        total_followers: 0,
        total_likes: 0,
        total_works: 0,
        
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
        
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };

    // 获取字段映射
    let get_field_name = |field: &str| -> &str {
        if let Some(mapping) = mapping {
            mapping.get(field).map(|s| s.as_str()).unwrap_or(field)
        } else {
            field
        }
    };

    // 查找字段索引的辅助函数
    let find_field_index = |field_name: &str| -> Option<usize> {
        headers.iter().position(|h| h == get_field_name(field_name))
    };

    // 解析必填字段
    if let Some(index) = find_field_index("id") {
        if let Some(value) = record.get(index) {
            if value.trim().is_empty() {
                errors.push(ImportError {
                    row: row_number,
                    field: "id".to_string(),
                    value: value.to_string(),
                    error: "ID不能为空".to_string(),
                });
            } else {
                doctor.id = value.trim().to_string();
            }
        }
    } else {
        errors.push(ImportError {
            row: row_number,
            field: "id".to_string(),
            value: "".to_string(),
            error: "缺少ID字段".to_string(),
        });
    }    // 解析其他必填字段
    macro_rules! parse_required_string {
        ($field:literal, $target:expr) => {
            if let Some(index) = find_field_index($field) {
                if let Some(value) = record.get(index) {
                    if value.trim().is_empty() {
                        errors.push(ImportError {
                            row: row_number,
                            field: $field.to_string(),
                            value: value.to_string(),
                            error: format!("{}不能为空", $field),
                        });
                    } else {
                        $target = Some(value.trim().to_string());
                    }
                }
            } else {
                errors.push(ImportError {
                    row: row_number,
                    field: $field.to_string(),
                    value: "".to_string(),
                    error: format!("缺少{}字段", $field),
                });
            }
        };
    }

    parse_required_string!("title", doctor.title);
    parse_required_string!("region", doctor.region);
    parse_required_string!("department", doctor.department);

    // 解析可选字符串字段
    if let Some(index) = find_field_index("agency_name") {
        if let Some(value) = record.get(index) {
            if !value.trim().is_empty() {
                doctor.agency_name = Some(value.trim().to_string());
            }
        }
    }

    // 解析数字字段
    macro_rules! parse_f64_field {
        ($field:literal, $target:expr) => {
            if let Some(index) = find_field_index($field) {
                if let Some(value) = record.get(index) {
                    match value.trim().parse::<f64>() {
                        Ok(num) => $target = Some(num),
                        Err(_) => {
                            errors.push(ImportError {
                                row: row_number,
                                field: $field.to_string(),
                                value: value.to_string(),
                                error: format!("{}必须是数字", $field),
                            });
                        }
                    }
                }
            }
        };
    }

    macro_rules! parse_i32_field {
        ($field:literal, $target:expr) => {
            if let Some(index) = find_field_index($field) {
                if let Some(value) = record.get(index) {
                    match value.trim().parse::<i32>() {
                        Ok(num) => $target = Some(num),
                        Err(_) => {
                            errors.push(ImportError {
                                row: row_number,
                                field: $field.to_string(),
                                value: value.to_string(),
                                error: format!("{}必须是数字", $field),
                            });
                        }
                    }
                }
            }
        };
    }

    // 必填整数字段
    macro_rules! parse_required_i32 {
        ($field:literal, $target:expr) => {
            if let Some(index) = find_field_index($field) {
                if let Some(value) = record.get(index) {
                    match value.trim().parse::<i32>() {
                        Ok(num) => $target = num,
                        Err(_) => {
                            errors.push(ImportError {
                                row: row_number,
                                field: $field.to_string(),
                                value: value.to_string(),
                                error: format!("{}必须是数字", $field),
                            });
                        }
                    }
                }
            }
        };
    }

    parse_f64_field!("agency_price", doctor.agency_price);
    parse_required_i32!("total_followers", doctor.total_followers);
    parse_required_i32!("total_likes", doctor.total_likes);
    parse_required_i32!("total_works", doctor.total_works);

    // 7天数据
    parse_i32_field!("likes_7d", doctor.likes_7d);
    parse_i32_field!("followers_7d", doctor.followers_7d);
    parse_i32_field!("shares_7d", doctor.shares_7d);
    parse_i32_field!("comments_7d", doctor.comments_7d);
    parse_i32_field!("works_7d", doctor.works_7d);

    // 15天数据
    parse_i32_field!("likes_15d", doctor.likes_15d);
    parse_i32_field!("followers_15d", doctor.followers_15d);
    parse_i32_field!("shares_15d", doctor.shares_15d);
    parse_i32_field!("comments_15d", doctor.comments_15d);
    parse_i32_field!("works_15d", doctor.works_15d);

    // 30天数据
    parse_i32_field!("likes_30d", doctor.likes_30d);
    parse_i32_field!("followers_30d", doctor.followers_30d);
    parse_i32_field!("shares_30d", doctor.shares_30d);
    parse_i32_field!("comments_30d", doctor.comments_30d);
    parse_i32_field!("works_30d", doctor.works_30d);

    // 人工评分 - 可选的f64字段，但转换为f32
    macro_rules! parse_f32_field {
        ($field:literal, $target:expr) => {
            if let Some(index) = find_field_index($field) {
                if let Some(value) = record.get(index) {
                    match value.trim().parse::<f64>() {
                        Ok(num) => $target = Some(num as f64),
                        Err(_) => {
                            errors.push(ImportError {
                                row: row_number,
                                field: $field.to_string(),
                                value: value.to_string(),
                                error: format!("{}必须是数字", $field),
                            });
                        }
                    }
                }
            }
        };
    }    parse_f32_field!("performance_score", doctor.performance_score);
    parse_f32_field!("affinity_score", doctor.affinity_score);
    parse_f32_field!("editing_score", doctor.editing_score);
    parse_f32_field!("video_quality_score", doctor.video_quality_score);
    parse_i64_field!("total_followers", doctor.total_followers);
    parse_i64_field!("total_likes", doctor.total_likes);
    parse_i64_field!("total_works", doctor.total_works);
    parse_i64_field!("likes_7d", doctor.likes_7d);
    parse_i64_field!("followers_15d", doctor.followers_15d);
    parse_i64_field!("shares_30d", doctor.shares_30d);
    parse_i64_field!("comments_30d", doctor.comments_30d);
    parse_i64_field!("works_7d", doctor.works_7d);    // 解析浮点数字段
    macro_rules! parse_f64_field {
        ($field:literal, $target:expr) => {
            if let Some(index) = find_field_index($field) {
                if let Some(value) = record.get(index) {
                    match value.trim().parse::<f64>() {
                        Ok(num) => {
                            if num >= 0.0 && num <= 100.0 {
                                $target = Some(num);
                            } else {
                                errors.push(ImportError {
                                    row: row_number,
                                    field: $field.to_string(),
                                    value: value.to_string(),
                                    error: format!("{}必须在0-100之间", $field),
                                });
                            }
                        },
                        Err(_) => {
                            errors.push(ImportError {
                                row: row_number,
                                field: $field.to_string(),
                                value: value.to_string(),
                                error: format!("{}必须是数字", $field),
                            });
                        }
                    }
                }
            }
        };
    }    parse_f64_field!("performance_score", doctor.performance_score);
    parse_f64_field!("editing_score", doctor.editing_score);
    parse_f64_field!("video_quality_score", doctor.video_quality_score);
    parse_f64_field!("affinity_score", doctor.affinity_score);

    if errors.is_empty() {
        Ok(doctor)
    } else {
        Err(errors)
    }
}

/// 保存医生数据到数据库
async fn save_doctor(
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
        // 只插入新记录，如果存在则跳过
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

/// 验证导入数据格式
pub async fn validate_import_data(
    import_req: web::Json<ImportRequest>,
) -> Result<HttpResponse> {
    let mut validation_result = ValidationResult {
        valid: true,
        message: "数据格式验证通过".to_string(),
        errors: Vec::new(),
        preview: Vec::new(),
    };

    match import_req.format.as_str() {
        "csv" => {
            validation_result = validate_csv_data(&import_req.data, &import_req.mapping);
        },
        "json" => {
            validation_result = validate_json_data(&import_req.data);
        },
        _ => {
            validation_result.valid = false;
            validation_result.message = "不支持的数据格式".to_string();
        }
    }

    Ok(HttpResponse::Ok().json(validation_result))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub message: String,
    pub errors: Vec<ImportError>,
    pub preview: Vec<Doctor>,
}

/// 验证 CSV 数据
fn validate_csv_data(
    csv_data: &str,
    mapping: &Option<HashMap<String, String>>,
) -> ValidationResult {
    let mut result = ValidationResult {
        valid: true,
        message: "CSV 数据格式验证通过".to_string(),
        errors: Vec::new(),
        preview: Vec::new(),
    };

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());

    let headers = match reader.headers() {
        Ok(headers) => headers.clone(),
        Err(e) => {
            result.valid = false;
            result.message = format!("CSV 格式错误: {}", e);
            return result;
        }
    };

    // 验证前5行数据作为预览
    for (row_index, record_result) in reader.records().enumerate() {
        if row_index >= 5 {
            break;
        }

        match record_result {
            Ok(record) => {
                match parse_csv_record(&headers, &record, mapping, row_index + 2) {
                    Ok(doctor) => {
                        result.preview.push(doctor);
                    },
                    Err(errors) => {
                        result.errors.extend(errors);
                        if result.errors.len() >= 10 {
                            break;
                        }
                    }
                }
            },
            Err(e) => {
                result.errors.push(ImportError {
                    row: row_index + 2,
                    field: "csv".to_string(),
                    value: "".to_string(),
                    error: format!("CSV 行解析错误: {}", e),
                });
            }
        }
    }

    if !result.errors.is_empty() {
        result.valid = false;
        result.message = format!("发现 {} 个验证错误", result.errors.len());
    }

    result
}

/// 验证 JSON 数据
fn validate_json_data(json_data: &str) -> ValidationResult {
    let mut result = ValidationResult {
        valid: true,
        message: "JSON 数据格式验证通过".to_string(),
        errors: Vec::new(),
        preview: Vec::new(),
    };

    match serde_json::from_str::<Vec<Doctor>>(json_data) {
        Ok(doctors) => {
            // 取前5条作为预览
            result.preview = doctors.into_iter().take(5).collect();
        },
        Err(e) => {
            result.valid = false;
            result.message = format!("JSON 格式错误: {}", e);
        }
    }

    result
}
