use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub debug: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "local".into());

        let settings = Config::builder()
            .add_source(File::with_name(&format!("settings/{}", environment)))
            .build()?;

        settings.try_deserialize()
    }
}
