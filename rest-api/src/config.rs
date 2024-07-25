//! This module provides configuration for the application.
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
use config::{Config, Environment};
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
    /// Loads configuration data from environment variables prefixed with `APP`.
    ///
    /// You can also set environment variables in a .env file for easier local development.
    pub fn load() -> Result<AppConfig> {
        dotenv::dotenv().ok();

        let config = Config::builder()
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("_"),
            )
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000)?
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;

        Ok(app_config)
    }
}
