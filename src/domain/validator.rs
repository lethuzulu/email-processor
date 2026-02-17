use crate::domain::{EmailSignature, ErrorCode, ValidationError, ValidationResult};
use chrono::Utc;
use once_cell::sync::Lazy;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use regex::Regex;

// compile regex once
static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

pub struct SignatureValidator;

impl SignatureValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, sig: &EmailSignature) -> ValidationResult {
        let mut errors = Vec::new();

        // validate name
        if sig.name.trim().is_empty() {
            errors.push(ValidationError {
                field: "name".to_string(),
                message: "Name is required".to_string(),
                code: ErrorCode::Required,
            })
        }

        if sig.name.len() > 100 {
            errors.push(ValidationError {
                field: "name".to_string(),
                message: "Name too long (max 100 characters)".to_string(),
                code: ErrorCode::TooLong,
            });
        }

        // validate email
        if !EMAIL_REGEX.is_match(&sig.email) {
            errors.push(ValidationError {
                field: "email".to_string(),
                message: "Invalid email format".to_string(),
                code: ErrorCode::InvalidFormat,
            });
        }

        // TODO: add phone validation & other field validation

        ValidationResult {
            signature_id: sig.id,
            valid: errors.is_empty(),
            errors,
            warnings: vec![],
            validated_at: Utc::now(),
        }
    }

    pub fn validate_batch(&self, sigs: &[EmailSignature]) -> Vec<ValidationResult> {
        sigs.par_iter().map(|sig| self.validate(sig)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_signature() {
        let validator = SignatureValidator::new();
        let sig = EmailSignature::builder()
            .name("John Doe")
            .email("john@example.com")
            .build();

        let result = validator.validate(&sig);
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_invalid_email() {
        let validator = SignatureValidator::new();
        let sig = EmailSignature::builder().email("invalid-email").build();

        let result = validator.validate(&sig);
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }
}
