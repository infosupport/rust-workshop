//! This module contains the logic to communicate with the database.

use crate::{
    config::DatabaseConfig,
    entity::{PagedResult, Task, TaskSummary},
    error::{AppError, Result},
};
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use tracing::{event, instrument, Level};

/// Creates a new database connection pool for the PostgreSQL database
/// based on the provided configuration.
///
/// You can change the configuration for the database connection in `config.toml`.
#[instrument]
pub async fn connect_db(config: &DatabaseConfig) -> Result<PgPool> {
    event!(Level::INFO, "Connecting to the database");

    let options = PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .database(&config.name)
        .username(&config.username)
        .password(&config.password);

    let pool = PgPoolOptions::new()
        .max_connections(12)
        .min_connections(2)
        .connect_with(options)
        .await?;

    Ok(pool)
}

/// Lists all todos in the database using paging.
///
/// This method executes two queries: one to fetch the items for the current page and another query to count the totals.
/// We implemented the [`sqlx::FromRow`] trait on [`crate::entity::TaskSummary`] to allow easy mapping of the query
/// results.
///
/// It's important to note that [`sqlx`] is not an ORM, so you'll need to write the SQL queries yourself. But you get strong
/// typing for result types so that's a good trade off when you want performance.
pub async fn list_todos(
    pool: &PgPool,
    page_index: i32,
    page_size: i32,
) -> Result<PagedResult<TaskSummary>> {
    let items = sqlx::query_as::<_, TaskSummary>(
        "SELECT id, title, description,completed FROM todos ORDER BY id LIMIT $1 OFFSET $2",
    )
    .bind(10)
    .bind(page_index * page_size)
    .fetch_all(pool)
    .await?;

    let total_count: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM todos")
        .fetch_one(pool)
        .await?;

    Ok(PagedResult {
        items,
        page_index,
        page_size,
        total_count,
    })
}

/// Finds a single todo item in the database by its ID.
///
/// We'll return [`std::result::Result::Ok`] with the [`Task`] if the todo is found,
/// otherwise we'll return [`std::result::Result::Err`] with the [`AppError::TodoNotFound`] error.
pub async fn find_todo(pool: &PgPool, id: i32) -> Result<Task> {
    let result: Option<Task> = sqlx::query_as::<_, Task>(
        "SELECT id, title, description, completed FROM todos WHERE id = $1 LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match result {
        Some(task) => Ok(task),
        None => Err(AppError::TodoNotFound),
    }
}

/// Inserts a new todo item in the database returning its ID.
#[instrument]
pub async fn insert_todo(pool: &PgPool, title: String, description: String) -> Result<i32> {
    let id: i32 = sqlx::query_scalar(
        "INSERT INTO todos (title, description, completed) VALUES ($1, $2, false) RETURNING id",
    )
    .bind(title)
    .bind(description)
    .fetch_one(pool)
    .await?;

    Ok(id)
}

/// Updates an existing todo item in the database.
pub async fn update_todo(
    pool: &PgPool,
    id: i32,
    title: String,
    description: String,
    completed: bool,
) -> Result<()> {
    let rows_affected =
        sqlx::query("UPDATE todos SET title = $1, description = $2, completed = $3 WHERE id = $4")
            .bind(title)
            .bind(description)
            .bind(completed)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::TodoNotFound);
    }

    Ok(())
}

pub async fn delete_todo(pool: &PgPool, id: i32) -> Result<()> {
    let rows_affected = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::TodoNotFound);
    }

    Ok(())
}
