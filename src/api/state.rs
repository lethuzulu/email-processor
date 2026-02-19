use crate::{infrastructure::Config, pipeline::PipelineManager};
use std::sync::Arc;

// shared app state
// cloned for each "worker", but Arc makes it cheap because of reference counting
#[derive(Clone)]
pub struct AppState {
    pub pipeline: Arc<PipelineManager>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            pipeline: Arc::new(PipelineManager::new()),
            config: Arc::new(config),
        }
    }
}
