// This file sets up the application routes and dispatches requests to the appropriate handlers.

use actix_web::{web, HttpResponse, Responder};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/example")
            .route(web::get().to(example_handler))
            .route(web::post().to(example_handler)),
    );
}

async fn example_handler() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}