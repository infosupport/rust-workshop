use std::sync::Arc;
use sqlx::PgPool;

pub struct AppState {
    pub db_connection_pool: PgPool,
}

impl AppState {
    pub fn new(db_connection_pool: PgPool) -> Arc<Self> {
        let app_state =  Self {
            db_connection_pool
        };

        Arc::new(app_state)
    }
}