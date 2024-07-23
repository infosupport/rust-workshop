//! This module contains the logic to communicate with the database.
//!
//! We don't use an ORM although we could use one. For example [Sea ORM](https://www.sea-ql.org/SeaORM/) is a good
//! choice for Rust applications. Alternatively you can use the [ormx](https://docs.rs/ormx/latest/ormx/) which is a
//! bit more low level but still very powerful.

use crate::{
    config::DatabaseConfig,
    entity::{PagedResult, Task, TaskSummary, User},
    error::{AppError, Result},
};
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use tracing::{event, instrument, Level};

/// Creates a new database connection pool for the PostgreSQL database
/// based on the provided configuration.
#[instrument(skip(config))]
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

/// List all tasks in the database for a specific user.
///
/// This method executes two queries: one to fetch the items for the current page and another query to count the totals.
/// We implemented the [`sqlx::FromRow`] trait on [`crate::entity::TaskSummary`] to allow easy mapping of the query
/// results.
///
/// It's important to note that [`sqlx`] is not an ORM, so you'll need to write the SQL queries yourself. But you get strong
/// typing for result types so that's a good trade off when you want performance.
pub async fn list_tasks(
    pool: &PgPool,
    user_id: i32,
    page_index: i32,
    page_size: i32,
) -> Result<PagedResult<TaskSummary>> {
    let items = sqlx::query_as::<_, TaskSummary>(
        "SELECT id, title, description,completed, date_created, date_modified FROM tasks WHERE user_id =$1 ORDER BY id LIMIT $2 OFFSET $3",
    )
    .bind(user_id)
    .bind(10)
    .bind(page_index * page_size)
    .fetch_all(pool)
    .await?;

    let total_count: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tasks")
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
pub async fn find_task(pool: &PgPool, user_id: i32, task_id: i32) -> Result<Task> {
    let result: Option<Task> = sqlx::query_as::<_, Task>(
        "SELECT id, title, description, completed, date_created, date_modified FROM tasks WHERE user_id = $1 AND id = $2 LIMIT 1",
    )
    .bind(user_id)
    .bind(task_id)
    .fetch_optional(pool)
    .await?;

    match result {
        Some(task) => Ok(task),
        None => Err(AppError::TaskNotFound),
    }
}

/// Inserts a new todo item in the database returning its ID.
///
/// We use the `RETURNING id` clause to return the ID of the newly inserted todo item.
#[instrument]
pub async fn insert_task(
    pool: &PgPool,
    user_id: i32,
    title: String,
    description: String,
) -> Result<i32> {
    let date_created = chrono::Utc::now();

    let id: i32 = sqlx::query_scalar(
        "INSERT INTO tasks (title, description, completed, user_id, date_created) VALUES ($1, $2, false, $3, $4) RETURNING id",
    )
    .bind(title)
    .bind(description)
    .bind(user_id)
    .bind(date_created)
    .fetch_one(pool)
    .await?;

    Ok(id)
}

/// Updates an existing todo item in the database.
///
/// We use the `rows_affected` method to check if the todo item was updated successfully.
/// If no rows were affected, we return an error with the [`AppError::TodoNotFound`] variant.
#[instrument]
pub async fn update_task(
    pool: &PgPool,
    user_id: i32,
    id: i32,
    title: String,
    description: String,
    completed: bool,
) -> Result<()> {
    let rows_affected =
        sqlx::query("UPDATE tasks SET title = $1, description = $2, completed = $3, date_modified = $4 WHERE user_id = $5 AND id = $6")
            .bind(title)
            .bind(description)
            .bind(completed)
            .bind(chrono::Utc::now())
            .bind(user_id)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::TaskNotFound);
    }

    Ok(())
}

/// Deletes a todo item from the database.
///
/// We use the `rows_affected` method to check if the todo item was deleted successfully.
/// If no rows were affected, we return an error with the [`AppError::TodoNotFound`] variant.
#[instrument]
pub async fn delete_task(pool: &PgPool, user_id: i32, id: i32) -> Result<()> {
    let rows_affected = sqlx::query("DELETE FROM tasks WHERE user_id = $1 AND id = $2")
        .bind(user_id)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::TaskNotFound);
    }

    Ok(())
}

/// Retrieves a single user from the database by its ID.
///
/// This method returns a [`Result`] with the [`User`] if the user is found.
/// Otherwise it returns an error.
#[instrument]
pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::UserNotFound)?;

    Ok(user)
}

/// Retrieves a single user from the database by its API key.
///
/// This method returns a [`Result`] with the [`User`] if the user is found.
/// Otherwise it returns an error.
pub async fn get_user_by_key(pool: &PgPool, api_key: &str) -> Result<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE api_key = $1")
        .bind(api_key)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::InvalidApiKey)?;

    Ok(user)
}

/// Inserts a new user profile in the database
///
/// This method returns the ID of the newly inserted user.
#[instrument(skip(api_key))]
pub async fn insert_user(pool: &PgPool, email_address: String, api_key: String) -> Result<i32> {
    let id: i32 = sqlx::query_scalar(
        "INSERT INTO users (email_address, api_key, date_created) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(email_address)
    .bind(api_key)
    .bind(chrono::Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(id)
}
