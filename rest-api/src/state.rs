//! This module contains the application state that is shared across the route handlers in the [`crate::web`] module.
//!
//! The application state is wrapped in a [`Arc`] object to allow it to be shared across multiple threads.
//!
//! The application state is created in the [`AppState::new`] function. This function takes the database connection pool
//! as an argument and returns an [`Arc`] object containing the application state.
use std::sync::Arc;

use sqlx::PgPool;

/// Contains information that must be shared across multiple web request handlers.
#[derive(Clone, Debug)]
pub struct AppState {
    /// The database connection pool to use for running database queries and updates.
    pub connection_pool: PgPool,
}

impl AppState {
    /// Creates a new instance of the application state.
    ///
    /// This method should only be called once per application. We assume that the object has a `'static` lifetime
    /// scope. This is because the application state is shared across multiple threads and needs to be `'static`.
    pub fn new(connection_pool: PgPool) -> Arc<AppState> {
        let app_state = AppState { connection_pool };
        Arc::new(app_state)
    }
}
