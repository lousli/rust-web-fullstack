/// 应用配置管理模块
/// 统一管理环境变量和应用配置

use std::env;

/// 应用配置结构体
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// 服务器配置
    pub server: ServerConfig,
    /// 数据库配置
    pub database: DatabaseConfig,
    /// 日志配置
    pub logging: LoggingConfig,
    /// 安全配置
    pub security: SecurityConfig,
    /// 性能配置
    pub performance: PerformanceConfig,
}

/// 服务器配置
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub worker_threads: usize,
}

/// 数据库配置（部分字段预留用于数据库连接优化）
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DatabaseConfig {
    pub url: String,
    #[allow(dead_code)]
    pub max_connections: u32,
    #[allow(dead_code)]
    pub connection_timeout: u64,
}

/// 日志配置（部分字段预留用于日志系统扩展）
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LoggingConfig {
    pub level: String,
    #[allow(dead_code)]
    pub enable_file_logging: bool,
    #[allow(dead_code)]
    pub log_directory: String,
}

/// 安全配置（部分字段预留用于安全功能扩展）
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SecurityConfig {
    #[allow(dead_code)]
    pub cors_allowed_origins: Vec<String>,
    pub jwt_secret: String,
    #[allow(dead_code)]
    pub api_key_header: String,
    #[allow(dead_code)]
    pub rate_limit_per_minute: u32,
}

/// 性能配置（部分字段预留用于性能优化扩展）
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PerformanceConfig {
    #[allow(dead_code)]
    pub max_upload_size: usize,
    #[allow(dead_code)]
    pub session_timeout: u64,
    pub health_check_enabled: bool,
    pub metrics_enabled: bool,
}

impl AppConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self, String> {
        // 尝试加载 .env 文件（开发环境）
        dotenv::dotenv().ok();

        Ok(AppConfig {
            server: ServerConfig {
                host: env::var("RUST_WEB_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("RUST_WEB_PORT")
                    .unwrap_or_else(|_| "8081".to_string())
                    .parse()
                    .map_err(|_| "Invalid RUST_WEB_PORT")?,
                worker_threads: env::var("WORKER_THREADS")
                    .unwrap_or_else(|_| "4".to_string())
                    .parse()
                    .map_err(|_| "Invalid WORKER_THREADS")?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "sqlite:./data/doctor_analysis.db".to_string()),
                max_connections: env::var("MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()
                    .map_err(|_| "Invalid MAX_CONNECTIONS")?,
                connection_timeout: env::var("CONNECTION_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .map_err(|_| "Invalid CONNECTION_TIMEOUT")?,
            },
            logging: LoggingConfig {
                level: env::var("RUST_LOG")
                    .unwrap_or_else(|_| "info".to_string()),
                enable_file_logging: env::var("ENABLE_FILE_LOGGING")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
                log_directory: env::var("LOG_DIRECTORY")
                    .unwrap_or_else(|_| "./logs".to_string()),
            },
            security: SecurityConfig {
                cors_allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "*".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                jwt_secret: env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "default-jwt-secret-change-in-production".to_string()),
                api_key_header: env::var("API_KEY_HEADER")
                    .unwrap_or_else(|_| "X-API-Key".to_string()),
                rate_limit_per_minute: env::var("RATE_LIMIT_PER_MINUTE")
                    .unwrap_or_else(|_| "60".to_string())
                    .parse()
                    .map_err(|_| "Invalid RATE_LIMIT_PER_MINUTE")?,
            },
            performance: PerformanceConfig {
                max_upload_size: env::var("MAX_UPLOAD_SIZE")
                    .unwrap_or_else(|_| "10485760".to_string()) // 10MB
                    .parse()
                    .map_err(|_| "Invalid MAX_UPLOAD_SIZE")?,
                session_timeout: env::var("SESSION_TIMEOUT")
                    .unwrap_or_else(|_| "3600".to_string()) // 1小时
                    .parse()
                    .map_err(|_| "Invalid SESSION_TIMEOUT")?,
                health_check_enabled: env::var("HEALTH_CHECK_ENABLED")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                metrics_enabled: env::var("METRICS_ENABLED")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
            },
        })
    }    /// 验证配置有效性
    pub fn validate(&self) -> Result<(), String> {
        // 验证端口范围（端口范围检查，忽略无用比较警告）
        #[allow(unused_comparisons)]
        if self.server.port < 1024 || self.server.port > 65535 {
            return Err("Port must be between 1024 and 65535".to_string());
        }

        // 验证工作线程数
        if self.server.worker_threads == 0 {
            return Err("Worker threads must be greater than 0".to_string());
        }

        // 验证数据库URL
        if self.database.url.is_empty() {
            return Err("Database URL cannot be empty".to_string());
        }

        // 验证JWT密钥（生产环境检查）
        if self.is_production() && self.security.jwt_secret == "default-jwt-secret-change-in-production" {
            return Err("JWT secret must be changed in production environment".to_string());
        }

        Ok(())
    }

    /// 检查是否为生产环境
    pub fn is_production(&self) -> bool {
        env::var("RUST_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase() == "production"
    }

    /// 检查是否为开发环境
    pub fn is_development(&self) -> bool {
        !self.is_production()
    }

    /// 获取服务器绑定地址
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// 打印配置摘要（不包含敏感信息）
    pub fn print_summary(&self) {
        println!("🔧 应用配置摘要:");
        println!("   服务器: {}:{}", self.server.host, self.server.port);
        println!("   工作线程: {}", self.server.worker_threads);
        println!("   日志级别: {}", self.logging.level);
        println!("   健康检查: {}", if self.performance.health_check_enabled { "启用" } else { "禁用" });
        println!("   指标收集: {}", if self.performance.metrics_enabled { "启用" } else { "禁用" });
        println!("   环境模式: {}", if self.is_production() { "生产" } else { "开发" });
        
        if self.is_development() {
            println!("   数据库: {}", self.database.url);
        }
    }
}

/// 全局配置实例
static mut GLOBAL_CONFIG: Option<AppConfig> = None;
static CONFIG_INIT: std::sync::Once = std::sync::Once::new();

/// 初始化全局配置
pub fn init_config() -> Result<(), String> {
    CONFIG_INIT.call_once(|| {
        match AppConfig::from_env() {
            Ok(config) => {
                if let Err(e) = config.validate() {
                    eprintln!("❌ 配置验证失败: {}", e);
                    std::process::exit(1);
                }
                unsafe {
                    GLOBAL_CONFIG = Some(config);
                }
            }
            Err(e) => {
                eprintln!("❌ 配置加载失败: {}", e);
                std::process::exit(1);
            }
        }
    });
    Ok(())
}

/// 获取全局配置
pub fn get_config() -> &'static AppConfig {
    unsafe {
        #[allow(static_mut_refs)]
        GLOBAL_CONFIG.as_ref().expect("Config not initialized. Call init_config() first.")
    }
}
