use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::error::Error;
use crate::configuration::DatabaseConfig;

pub async fn connect_db(config: DatabaseConfig) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .connect(config.to_url().as_str()).await
}
