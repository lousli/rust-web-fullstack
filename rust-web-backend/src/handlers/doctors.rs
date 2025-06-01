use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use crate::models::{Doctor, ApiResponse, QueryParams};

/// 模拟医生数据
fn get_mock_doctors() -> Vec<Doctor> {
    vec![
        Doctor {
            id: 1,
            doctor_id: "DOC001".to_string(),
            name: "张伟明".to_string(),
            title: "主任医师".to_string(),
            region: "北京".to_string(),
            department: "心内科".to_string(),
            institution: "北京协和医院".to_string(),
            account_type: "头部".to_string(),
            created_at: Utc::now(),
        },
        Doctor {
            id: 2,
            doctor_id: "DOC002".to_string(),
            name: "李小红".to_string(),
            title: "副主任医师".to_string(),
            region: "上海".to_string(),
            department: "神经内科".to_string(),
            institution: "上海华山医院".to_string(),
            account_type: "腰部".to_string(),
            created_at: Utc::now(),
        },
        Doctor {
            id: 3,
            doctor_id: "DOC003".to_string(),
            name: "王建国".to_string(),
            title: "主治医师".to_string(),
            region: "广州".to_string(),
            department: "骨科".to_string(),
            institution: "中山大学附属第一医院".to_string(),
            account_type: "尾部".to_string(),
            created_at: Utc::now(),
        },
        Doctor {
            id: 4,
            doctor_id: "DOC004".to_string(),
            name: "陈雅文".to_string(),
            title: "主任医师".to_string(),
            region: "深圳".to_string(),
            department: "妇产科".to_string(),
            institution: "深圳市人民医院".to_string(),
            account_type: "头部".to_string(),
            created_at: Utc::now(),
        },
        Doctor {
            id: 5,
            doctor_id: "DOC005".to_string(),
            name: "刘志强".to_string(),
            title: "副主任医师".to_string(),
            region: "杭州".to_string(),
            department: "儿科".to_string(),
            institution: "浙江大学医学院附属儿童医院".to_string(),
            account_type: "腰部".to_string(),
            created_at: Utc::now(),
        },
        Doctor {
            id: 6,
            doctor_id: "DOC006".to_string(),
            name: "赵美丽".to_string(),
            title: "主治医师".to_string(),
            region: "成都".to_string(),
            department: "眼科".to_string(),
            institution: "四川大学华西医院".to_string(),
            account_type: "尾部".to_string(),
            created_at: Utc::now(),
        },
        Doctor {
            id: 7,
            doctor_id: "DOC007".to_string(),
            name: "孙立华".to_string(),
            title: "主任医师".to_string(),
            region: "西安".to_string(),
            department: "呼吸内科".to_string(),
            institution: "西安交通大学第一附属医院".to_string(),
            account_type: "头部".to_string(),
            created_at: Utc::now(),
        },
        Doctor {
            id: 8,
            doctor_id: "DOC008".to_string(),
            name: "周晓明".to_string(),
            title: "副主任医师".to_string(),
            region: "武汉".to_string(),
            department: "消化内科".to_string(),
            institution: "华中科技大学同济医学院附属同济医院".to_string(),
            account_type: "腰部".to_string(),
            created_at: Utc::now(),
        },
    ]
}

/// 获取医生列表
pub async fn get_doctors(query: web::Query<QueryParams>) -> Result<HttpResponse> {
    let mut doctors = get_mock_doctors();
    
    // 应用筛选条件
    if let Some(ref name) = query.name {
        doctors.retain(|d| d.name.contains(name));
    }
    
    if let Some(ref department) = query.department {
        doctors.retain(|d| d.department == *department);
    }
    
    if let Some(ref region) = query.region {
        doctors.retain(|d| d.region == *region);
    }
    
    if let Some(ref account_type) = query.account_type {
        doctors.retain(|d| d.account_type == *account_type);
    }
    
    let total = doctors.len() as u32;
    
    // 分页
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(doctors.len());
    
    let paginated_doctors = if start < doctors.len() {
        doctors[start..end].to_vec()
    } else {
        vec![]
    };
    
    Ok(HttpResponse::Ok().json(ApiResponse::success_with_total(paginated_doctors, total)))
}

/// 获取单个医生信息
pub async fn get_doctor(path: web::Path<u32>) -> Result<HttpResponse> {
    let doctor_id = path.into_inner();
    let doctors = get_mock_doctors();
    
    if let Some(doctor) = doctors.into_iter().find(|d| d.id == doctor_id) {
        Ok(HttpResponse::Ok().json(ApiResponse::success(doctor)))
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("医生未找到".to_string())))
    }
}

/// 创建医生信息
pub async fn create_doctor(doctor: web::Json<Doctor>) -> Result<HttpResponse> {
    // 在实际应用中，这里会保存到数据库
    println!("Creating doctor: {:?}", doctor);
    Ok(HttpResponse::Created().json(ApiResponse::success(doctor.into_inner())))
}

/// 更新医生信息
pub async fn update_doctor(path: web::Path<u32>, doctor: web::Json<Doctor>) -> Result<HttpResponse> {
    let doctor_id = path.into_inner();
    // 在实际应用中，这里会更新数据库
    println!("Updating doctor {}: {:?}", doctor_id, doctor);
    Ok(HttpResponse::Ok().json(ApiResponse::success(doctor.into_inner())))
}

/// 删除医生信息
pub async fn delete_doctor(path: web::Path<u32>) -> Result<HttpResponse> {
    let doctor_id = path.into_inner();
    // 在实际应用中，这里会从数据库删除
    println!("Deleting doctor: {}", doctor_id);
    Ok(HttpResponse::Ok().json(ApiResponse::success("删除成功".to_string())))
}
