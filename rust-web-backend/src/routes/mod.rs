use actix_web::web;
use crate::handlers::{doctors, scores, weights, import, reports, medical_scoring};
use crate::monitoring;

/// 配置所有路由
/// 
/// 设置应用程序路由并将请求分派给相应的处理程序
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // API 路由组
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/doctors")
                    .route("", web::get().to(doctors::get_doctors))
                    .route("", web::post().to(doctors::create_doctor))
                    .route("/{id}", web::get().to(doctors::get_doctor))
                    .route("/{id}", web::put().to(doctors::update_doctor))
                    .route("/{id}", web::delete().to(doctors::delete_doctor))
                    .route("/import", web::post().to(doctors::import_csv))
                    .route("/export", web::get().to(doctors::export_csv))
                    .route("/statistics", web::get().to(doctors::get_statistics))
            )
            .service(
                web::scope("/scores")
                    .route("", web::get().to(scores::get_scores))
                    .route("/calculate", web::post().to(scores::calculate_scores))
                    .route("/ranking", web::get().to(scores::get_ranking))
                    .route("/comparison", web::post().to(scores::compare_doctors))
                    .route("/trends/{doctor_id}", web::get().to(scores::get_score_trends))
                    .route("/recommendations", web::get().to(scores::get_recommendations))
                    .route("/statistics", web::get().to(scores::get_score_statistics))
                    .route("/doctor/{id}", web::get().to(scores::get_doctor_score))
            )
            .service(
                web::scope("/weight-configs")
                    .route("", web::get().to(weights::get_weight_configs))
                    .route("", web::post().to(weights::create_weight_config))
                    .route("/active", web::get().to(weights::get_active_weight_config))
                    .route("/{id}/activate", web::post().to(weights::activate_weight_config))
                    .route("/validate", web::post().to(weights::validate_weight_config))
                    // 医疗专用权重配置API
                    .route("/medical/presets", web::get().to(weights::get_medical_weight_presets))
                    .route("/medical", web::post().to(weights::create_medical_weight_config))
                    .route("/medical/analyze", web::post().to(weights::analyze_weight_impact))
            )
            .service(
                web::scope("/medical-scoring")
                    .route("/recalculate", web::post().to(medical_scoring::recalculate_medical_scores))
                    .route("/doctor/{id}", web::get().to(medical_scoring::get_doctor_medical_score))
                    .route("/details/{id}", web::get().to(medical_scoring::get_medical_score_details))
            )
            .service(
                web::scope("/import")
                    .route("/template", web::get().to(import::get_import_template))
                    // .route("/validate", web::post().to(import::validate_import_data))
                    // .route("/doctors", web::post().to(import::import_doctors))
            )
            .service(
                web::scope("/reports")
                    .route("/overview", web::post().to(reports::generate_overview_report))
                    .route("/ranking", web::post().to(reports::generate_ranking_report))
                    .route("/analysis", web::post().to(reports::generate_analysis_report))
                    .route("/export/csv", web::post().to(reports::export_report_csv))
            )
            // 监控和健康检查端点
            .service(
                web::scope("/monitoring")
                    .route("/health", web::get().to(monitoring::health_check))
                    .route("/health/detailed", web::get().to(monitoring::detailed_health_check))
                    .route("/metrics", web::get().to(monitoring::system_metrics))
                    .route("/readiness", web::get().to(monitoring::readiness_check))
                    .route("/liveness", web::get().to(monitoring::liveness_check))
            )
            // 保持原有的简单健康检查端点以保持向后兼容
            .route("/health", web::get().to(monitoring::health_check))
    );
}