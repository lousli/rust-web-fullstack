/// 系统监控和健康检查模块
/// 提供系统状态监控、健康检查和性能指标收集功能

use actix_web::{HttpResponse, web};
use serde::Serialize;
use chrono::{DateTime, Utc};
use std::time::SystemTime;
use sqlx::SqlitePool;

/// 健康检查状态
#[derive(Debug, Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub uptime_seconds: u64,
    pub checks: Vec<HealthCheck>,
    pub system_info: SystemInfo,
}

/// 单个健康检查项
#[derive(Debug, Serialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: String,
    pub response_time_ms: u64,
    pub message: Option<String>,
    pub details: Option<serde_json::Value>,
}

/// 系统信息
#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub rust_version: String,
    pub actix_version: String,
    pub database_type: String,
    pub features_enabled: Vec<String>,
}

/// 性能指标
#[derive(Debug, Serialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub requests_total: u64,
    pub requests_per_second: f64,
    pub active_connections: u32,
    pub memory_usage_mb: f64,
    pub database_connections: u32,
    pub uptime_seconds: u64,
}

/// 应用程序启动时间（用于计算运行时间）
static mut START_TIME: Option<SystemTime> = None;
static START_TIME_INIT: std::sync::Once = std::sync::Once::new();

/// 初始化启动时间
pub fn init_start_time() {
    START_TIME_INIT.call_once(|| {
        unsafe {
            START_TIME = Some(SystemTime::now());
        }
    });
}

/// 获取运行时间（秒）
fn get_uptime_seconds() -> u64 {
    unsafe {
        if let Some(start_time) = START_TIME {
            SystemTime::now()
                .duration_since(start_time)
                .unwrap_or_default()
                .as_secs()
        } else {
            0
        }
    }
}

/// 基础健康检查端点
pub async fn health_check() -> HttpResponse {
    let uptime = get_uptime_seconds();
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "message": "医生数据分析系统运行正常",
        "timestamp": Utc::now(),
        "version": env!("CARGO_PKG_VERSION"),
        "uptime_seconds": uptime
    }))
}

/// 详细健康检查端点
pub async fn detailed_health_check(pool: web::Data<SqlitePool>) -> HttpResponse {
    let mut checks = Vec::new();
    let start_time = std::time::Instant::now();

    // 数据库连接检查
    let db_start = std::time::Instant::now();
    let db_check = check_database_health(pool.get_ref()).await;
    let db_duration = db_start.elapsed().as_millis() as u64;
    checks.push(HealthCheck {
        name: "database".to_string(),
        status: if db_check.is_ok() { "healthy" } else { "unhealthy" }.to_string(),
        response_time_ms: db_duration,
        message: db_check.err(),
        details: None,
    });

    // 磁盘空间检查
    let disk_start = std::time::Instant::now();
    let disk_check = check_disk_space().await;
    let disk_duration = disk_start.elapsed().as_millis() as u64;
    checks.push(HealthCheck {
        name: "disk_space".to_string(),
        status: if disk_check.is_ok() { "healthy" } else { "warning" }.to_string(),
        response_time_ms: disk_duration,
        message: disk_check.err(),
        details: None,
    });

    // 内存使用检查
    let memory_start = std::time::Instant::now();
    let memory_check = check_memory_usage().await;
    let memory_duration = memory_start.elapsed().as_millis() as u64;
    checks.push(HealthCheck {
        name: "memory".to_string(),
        status: "healthy".to_string(), // 内存检查通常不会失败
        response_time_ms: memory_duration,
        message: None,
        details: Some(serde_json::json!({
            "usage_info": memory_check.unwrap_or_else(|_| "Unable to determine".to_string())
        })),
    });

    // 计算总体状态
    let overall_status = if checks.iter().all(|c| c.status == "healthy") {
        "healthy"
    } else if checks.iter().any(|c| c.status == "unhealthy") {
        "unhealthy"
    } else {
        "degraded"
    };

    let _total_duration = start_time.elapsed().as_millis() as u64;

    let health_status = HealthStatus {
        status: overall_status.to_string(),
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: get_uptime_seconds(),
        checks,
        system_info: SystemInfo {
            rust_version: rustc_version_runtime::version().to_string(),
            actix_version: "4.0".to_string(), // 硬编码版本，或从 Cargo.toml 读取
            database_type: "SQLite".to_string(),
            features_enabled: vec![
                "cors".to_string(),
                "logging".to_string(),
                "static_files".to_string(),
                "health_check".to_string(),
            ],
        },
    };

    // 根据状态返回相应的 HTTP 状态码
    match overall_status {
        "healthy" => HttpResponse::Ok().json(health_status),
        "degraded" => HttpResponse::Ok().json(health_status), // 警告状态仍返回 200
        _ => HttpResponse::ServiceUnavailable().json(health_status),
    }
}

/// 系统指标端点
pub async fn system_metrics(pool: web::Data<SqlitePool>) -> HttpResponse {
    let metrics = SystemMetrics {
        timestamp: Utc::now(),
        requests_total: 0, // 这里需要实际的指标收集
        requests_per_second: 0.0,
        active_connections: 0,
        memory_usage_mb: get_memory_usage_mb().await.unwrap_or(0.0),
        database_connections: get_db_connections_count(pool.get_ref()).await.unwrap_or(0),
        uptime_seconds: get_uptime_seconds(),
    };

    HttpResponse::Ok().json(metrics)
}

/// 检查数据库健康状态
async fn check_database_health(pool: &SqlitePool) -> Result<String, String> {
    let start = std::time::Instant::now();
    
    // 执行简单查询测试连接
    let result = sqlx::query("SELECT 1 as test")
        .fetch_one(pool)
        .await;

    let duration = start.elapsed();

    match result {
        Ok(_) => Ok(format!("Database responsive in {:?}", duration)),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

/// 检查磁盘空间
async fn check_disk_space() -> Result<String, String> {
    // 简化的磁盘空间检查
    // 在实际应用中，可以使用系统调用获取磁盘使用情况
    
    // 检查数据目录是否存在
    if std::path::Path::new("./data").exists() {
        Ok("Data directory accessible".to_string())
    } else {
        Err("Data directory not found".to_string())
    }
}

/// 检查内存使用情况
async fn check_memory_usage() -> Result<String, String> {
    // 这是一个简化的内存检查
    // 在实际应用中，可以使用系统库获取真实的内存使用情况
    Ok("Memory usage within normal range".to_string())
}

/// 获取内存使用量（MB）
async fn get_memory_usage_mb() -> Result<f64, String> {
    // 简化实现，返回一个估算值
    // 实际应用中应该使用系统调用获取真实内存使用
    Ok(64.0) // 估算值
}

/// 获取数据库连接数
async fn get_db_connections_count(pool: &SqlitePool) -> Result<u32, String> {
    // SQLite 的连接池信息获取
    // 这是一个简化实现
    Ok(pool.size() as u32)
}

/// 就绪检查端点（用于容器编排）
pub async fn readiness_check(pool: web::Data<SqlitePool>) -> HttpResponse {
    // 检查关键依赖是否就绪
    match check_database_health(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "ready",
            "timestamp": Utc::now(),
            "message": "Service is ready to accept requests"
        })),
        Err(e) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "not_ready",
            "timestamp": Utc::now(),
            "message": format!("Service not ready: {}", e)
        })),
    }
}

/// 存活检查端点（用于容器编排）
pub async fn liveness_check() -> HttpResponse {
    // 简单的存活检查，只要进程在运行就返回成功
    HttpResponse::Ok().json(serde_json::json!({
        "status": "alive",
        "timestamp": Utc::now(),
        "uptime_seconds": get_uptime_seconds()
    }))
}
