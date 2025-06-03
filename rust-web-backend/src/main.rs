use actix_files::Files;
use actix_web::{
    middleware::{Logger, DefaultHeaders},
    web, App, HttpServer
};
use actix_cors::Cors;
use local_ip_address;

mod routes;
mod handlers;
mod models;
mod database;
mod algorithms;
mod config;
mod monitoring;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化监控启动时间
    monitoring::init_start_time();
    
    // 初始化配置
    config::init_config().expect("Failed to initialize config");
    let config = config::get_config();
    
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(&config.logging.level));
    
    println!("🚀 启动医生数据分析系统后端服务器...");
    config.print_summary();
    
    // 初始化数据库
    println!("📊 初始化数据库连接...");
    let pool = match database::setup_database().await {
        Ok(pool) => {
            println!("✅ 数据库连接成功");
            pool
        },
        Err(e) => {
            eprintln!("❌ 数据库连接失败: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("📊 本地访问: http://127.0.0.1:{}", config.server.port);
    println!("📱 手机访问: http://0.0.0.0:{} (使用你的电脑IP地址)", config.server.port);
    println!("🔗 API 文档: http://127.0.0.1:{}/api", config.server.port);
    
    // 获取本机IP地址提示
    if let Ok(local_ip) = local_ip_address::local_ip() {
        println!("🌐 建议手机访问地址: http://{}:{}", local_ip, config.server.port);
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("X-Version", "1.0")))
            .configure(routes::configure_routes)
            // 提供静态文件服务，映射 / 前缀到 frontend 目录
            .service(Files::new("/", "./frontend").index_file("index.html"))
    })
    .workers(config.server.worker_threads)
    .bind(&config.bind_address())?
    .run()
    .await
}