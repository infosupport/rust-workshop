use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn to_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct ApplicationConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

impl ApplicationConfig {
    pub fn load() -> Result<ApplicationConfig, ConfigError> {
        let mut builder = Config::builder()
            .add_source(File::new("settings", FileFormat::Toml))
            .add_source(Environment::with_prefix("APP"));

        // Provide a few sensible defaults for the database configuration
        // These settings allow you to docker run the postgres docker images with a very basic password.
        // I suggest you change your password in any case.
        builder = builder.set_default("database.port", 5432)?;
        builder = builder.set_default("database.host", "localhost")?;
        builder = builder.set_default("database.username", "postgres")?;
        builder = builder.set_default("database.password", "postgres")?;
        builder = builder.set_default("database.database", "postgres")?;

        // Provide a few sensible defaults for the server configuration
        builder = builder.set_default("server.host", "0.0.0.0")?;
        builder = builder.set_default("server.port", 3000)?;

        let config = builder.build()?;
        let app_config = config.try_deserialize::<ApplicationConfig>()?;

        Ok(app_config)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_url() {
        let db_config = DatabaseConfig {
            username: "postgres".to_string(),
            password: "password".to_string(),
            port: 5432,
            host: "localhost".to_string(),
            database: "todo-api".to_string(),
        };

        assert_eq!(
            db_config.to_url(),
            "postgres://postgres:password@localhost:5432/todo-api"
        );
    }

    #[test]
    fn test_load() {
        let app_config = ApplicationConfig::load().unwrap();

        assert_eq!(app_config.database.username, "postgres");
        assert_eq!(app_config.database.password, "postgres");
        assert_eq!(app_config.database.port, 5432);
        assert_eq!(app_config.database.host, "localhost");
        assert_eq!(app_config.database.database, "postgres");
        assert_eq!(app_config.server.host, "0.0.0.0");
        assert_eq!(app_config.server.port, 3000);
    }
}