use std::sync::Arc;

use crate::domain::{EmailSignature, SignatureValidator, ValidationResult};

pub struct PipelineManager {
    validator: Arc<SignatureValidator>,
}

impl PipelineManager {
    pub fn new() -> Self {
        Self {
            validator: Arc::new(SignatureValidator::new()),
        }
    }

    pub async fn process_single(&self, sig: EmailSignature) -> ValidationResult {
        self.validator.validate(&sig)
    }

    pub async fn process_batch(&self, sigs: Vec<EmailSignature>) -> Vec<ValidationResult> {
        self.validator.validate_batch(&sigs)
    }
}
