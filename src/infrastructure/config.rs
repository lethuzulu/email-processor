use config::{Config as ConfigLoader, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pipeline: PipelineConfig,
    pub observability: ObservabilityConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PipelineConfig {
    pub workers: usize,
    pub max_batch_size: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ObservabilityConfig {
    pub log_level: String,
    pub log_format: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config = ConfigLoader::builder()
            // load defauts first
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("server.workers", 4)?
            .set_default("pipeline.workers", 4)?
            .set_default("pipeline.max_batch_size", 1000)?
            .set_default("observability.log_level", "info")?
            .set_default("observability.log_format", "json")?
            .add_source(File::with_name("config").required(false)) // add config from external source (file)
            .build()?;
        Ok(config.try_deserialize()?)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.server.port == 0 {
            anyhow::bail!("Server port cannot be 0");
        }

        if self.server.workers == 0 {
            anyhow::bail!("Server workers cannot be 0");
        }

        if self.pipeline.max_batch_size == 0 {
            anyhow::bail!("Max batch size cannot be 0");
        }
        Ok(())
    }
}
