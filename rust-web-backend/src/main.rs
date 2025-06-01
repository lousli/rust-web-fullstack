use actix_files::Files;
use actix_web::{
    middleware::{Logger, DefaultHeaders},
    App, HttpServer
};
use actix_cors::Cors;

mod routes;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    println!("🚀 启动医生数据分析系统后端服务器...");
    println!("📊 服务地址: http://127.0.0.1:8080");
    println!("🔗 API 文档: http://127.0.0.1:8080/api");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("X-Version", "1.0")))
            .configure(routes::configure_routes)
            // 提供静态文件服务，映射 / 前缀到 frontend 目录
            .service(Files::new("/", "./frontend").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}