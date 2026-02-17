use actix_web::http::StatusCode;
use anyhow::Error as AnyhowError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum AppError {
    Config(AnyhowError),
    Validation(String),
    Render(String),
    Internal(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(e) => write!(f, "Configuration error: {}", e),
            Self::Validation(e) => write!(f, "Validation error: {}", e),
            Self::Render(e) => write!(f, "Rendering error: {}", e),
            Self::Internal(e) => write!(f, "Internal eror: {}", e),
        }
    }
}

impl From<AnyhowError> for AppError {
    fn from(e: AnyhowError) -> Self {
        Self::Config(e)
    }
}

// impl Error for AppError {}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let status = match self {
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        actix_web::HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}
