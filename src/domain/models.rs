use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct EmailSignature {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub title: Option<String>,
    pub template_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationResult {
    pub signature_id: Uuid,
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
    pub validated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: ErrorCode,
}

#[derive(Debug, Clone, Serialize)]
pub enum ErrorCode {
    Required,
    InvalidFormat,
    TooLong,
    TooShort,
}

impl EmailSignature {
    // create a builder for testing
    pub fn builder() -> EmailSignatureBuilder {
        EmailSignatureBuilder::default()
    }
}

// builder pattern. will be used for test data creation
#[derive(Default)]
pub struct EmailSignatureBuilder {
    name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    company: Option<String>,
    title: Option<String>,
    template_id: Option<Uuid>,
}

impl EmailSignatureBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }

    pub fn company(mut self, company: impl Into<String>) -> Self {
        self.company = Some(company.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn template_id(mut self, id: Uuid) -> Self {
        self.template_id = Some(id);
        self
    }

    pub fn build(self) -> EmailSignature {
        EmailSignature {
            id: Uuid::new_v4(),
            name: self.name.unwrap_or_else(|| "John Doe".to_string()),
            email: self.email.unwrap_or_else(|| "john@example.com".to_string()),
            phone: self.phone,
            company: self.company,
            title: self.title,
            template_id: self.template_id.unwrap_or_else(Uuid::new_v4),
            created_at: Utc::now(),
        }
    }
}
