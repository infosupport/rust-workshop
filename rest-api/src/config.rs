//! This module provides configuration for the application.
//!
//! The configuration is loaded from the `config.toml` file and environment variables.
//! You can consider `config.toml` as the default configuration and environment variables as overrides.
//!
//! To configure the application with environment variables you'll need to prefix them with `APP`.
//! For example `APP_DATABASE_HOST=localhost` will set the database host to `localhost`.
//!
//! Configuration in Rust is often loaded from environment variables to make it easier to configure
//! the application in environments like Kubernetes or Docker.
//!
//! However, it can be useful to use a configuration file if you want to store the configuration
//! in source control or a configuration management system.

use crate::error::Result;
use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;

/// Database configuration data structure.
/// This is used to configure the database connection.
#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
}

/// Server configuration data structure.
/// This is used to configure the server's host and port.
#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    /// Translates the server configuration into an address string.
    /// This is used by the main application to bind the server to a specific address.
    pub fn to_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Root configuration data structure.
#[derive(Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

impl AppConfig {
    /// Loads configuration data from `config.toml` and environment variables prefixed with `APP`.
    /// Environment variables take precedence over the configuration file.
    pub fn load() -> Result<AppConfig> {
        let config = Config::builder()
            .add_source(File::with_name("config").format(FileFormat::Toml))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;

        Ok(app_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_config_returns_settings() {
        AppConfig::load().unwrap();
    }
}