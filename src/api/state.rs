use crate::{domain::SignatureValidator, infrastructure::Config};
use std::sync::Arc;

// shared app state
// cloned for each "worker", but Arc makes it cheap because of reference counting
#[derive(Clone)]
pub struct AppState {
    pub validator: Arc<SignatureValidator>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            validator: Arc::new(SignatureValidator::new()),
            config: Arc::new(config),
        }
    }
}
