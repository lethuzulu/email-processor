use config::{Config as ConfigLoader, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    server: ServerConfig
}



#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize
}



impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config = ConfigLoader::builder()
            // load defauts first
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("server.workers", 4)?
            .add_source(File::with_name("config").required(false)) // add config from external source (file)
            .build()?;
        Ok(config.try_deserialize()?)
    }
}





