use actix_files::Files;
use actix_web::{App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the Rust web backend server...");

    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes)
            // 提供静态文件服务，映射 / 前缀到 frontend/src 目录
            .service(Files::new("/", "../frontend/src").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}