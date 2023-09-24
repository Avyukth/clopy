use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub base_dir: PathBuf,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::new();

        config.merge(File::with_name("config/default"))?;

        let env = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        config.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        config.merge(File::with_name("config/local").required(false))?;

        config.merge(Environment::with_prefix("APP"))?;

        config.try_into()
    }
}
