use actix_web::web;

use super::{handlers, middleware::Metrics};

// Configure all API routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check
        .route("/health", web::get().to(handlers::health))
        // API v1 routes
        .service(
            web::scope("/api/v1/signatures")
                .wrap(Metrics)
                .route("/validate", web::post().to(handlers::validate_signature))
                .route("/validate-batch", web::post().to(handlers::validate_batch)),
        );
}
