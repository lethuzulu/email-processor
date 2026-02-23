use actix_web::{HttpResponse, Result, web};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::info;

use crate::api::state::AppState;
use crate::domain::models::*;
use crate::error::AppError;

// health check endpoint
pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now()
    })))
}

// single signature validation
pub async fn validate_signature(
    state: web::Data<AppState>,
    signature: web::Json<EmailSignature>,
) -> Result<HttpResponse, AppError> {
    let start = Instant::now();

    info!(
        signature_id = %signature.id,
        email = %signature.email,
        "Validating signature"
    );

    // validate the signature via pipeline
    let result = state.pipeline.process_single(signature.into_inner()).await;

    let duration = start.elapsed();
    info!(
        signature_id = %result.signature_id,
        valid = result.valid,
        duration_ms = duration.as_millis(),
        "Validation complete"
    );

    crate::infrastructure::metrics::record_validation(result.valid);

    Ok(HttpResponse::Ok().json(result))
}

// batch validation
pub async fn validate_batch(
    state: web::Data<AppState>,
    request: web::Json<BatchValidateResult>,
) -> Result<HttpResponse, AppError> {
    let start = Instant::now();
    let batch_size = request.signatures.len();

    // validate batch size
    if batch_size > state.config.pipeline.max_batch_size {
        return Err(AppError::Validation(format!(
            "Batch size {} exceeds maximum of {}",
            batch_size, state.config.pipeline.max_batch_size
        )));
    }

    info!(batch_size = batch_size, "Processing batch validation");

    // use pipeline for batch validation
    let results = state.pipeline.process_batch(request.into_inner().signatures).await;

    // calculate summary
    let valid_count = results.iter().filter(|r| r.valid).count();
    let invalid_count = batch_size - valid_count;
    let duration = start.elapsed();

    info!(
        batch_size = batch_size,
        valid = valid_count,
        invalid = invalid_count,
        duration_ms = duration.as_millis(),
        "Batch validation complete"
    );

    for result in &results {
        crate::infrastructure::metrics::record_validation(result.valid);
    }

    let response = BatchValidateResponse {
        results,
        summary: BatchSummary {
            total: batch_size,
            valid: valid_count,
            invalid: invalid_count,
            processing_time_ms: duration.as_millis(),
        },
    };

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Debug, Deserialize)]
pub struct BatchValidateRequest {
    pub signatures: Vec<EmailSignature>,
}

#[derive(Debug, Deserialize)]
pub struct BatchValidateResult {
    pub signatures: Vec<EmailSignature>,
}

#[derive(Debug, Serialize)]
pub struct BatchValidateResponse {
    pub results: Vec<ValidationResult>,
    pub summary: BatchSummary,
}

#[derive(Debug, Serialize)]
pub struct BatchSummary {
    pub total: usize,
    pub valid: usize,
    pub invalid: usize,
    pub processing_time_ms: u128,
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::Config;

    fn create_test_state() -> web::Data<AppState> {
        let config = Config::load().unwrap();
        web::Data::new(AppState::new(config))
    }

    #[actix_web::test]
    async fn test_health() {
        let resp = health().await.unwrap();
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_validate_signature_valid() {
        let state = create_test_state();
        let sig = EmailSignature::builder()
            .name("John Doe")
            .email("john@example.com")
            .build();

        let resp = validate_signature(state, web::Json(sig)).await.unwrap();

        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_validate_signature_invalid() {
        let state = create_test_state();
        let sig = EmailSignature::builder()
            .email("invalid-email") // No @ symbol
            .build();

        let resp = validate_signature(state, web::Json(sig)).await.unwrap();

        assert_eq!(resp.status(), 200);
        // Note: Status is 200 because validation ran successfully
        // The ValidationResult.valid field will be false
    }

    #[actix_web::test]
    async fn test_validate_batch() {
        let state = create_test_state();
        let request = BatchValidateResult {
            signatures: vec![
                EmailSignature::builder().email("valid@example.com").build(),
                EmailSignature::builder().email("invalid").build(),
            ],
        };

        let resp = validate_batch(state, web::Json(request)).await.unwrap();

        assert_eq!(resp.status(), 200);
    }
}
