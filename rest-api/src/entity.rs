//! This module defines the data structures that represent the entities in the application.
//!
//! We mainly use these for loading/saving data in the database. You can also find these entities in the responses
//! of the web interface of the application.
//!
//! Note that not all fields are serialized by the API. For example, the generated API key for a user is not serialized.

use serde::Serialize;
use sqlx::FromRow;

/// Defines the structure of a paged resultset
#[derive(Serialize)]
pub struct PagedResult<T> {
    /// Items retrieved from the database
    pub items: Vec<T>,

    /// Page index that was retrieved
    pub page_index: i32,

    /// Number of items per page
    pub page_size: i32,

    /// The total number of items retrieved
    pub total_count: i64,
}

/// Defines the data structure for a task.
#[derive(FromRow, Serialize)]
pub struct Task {
    // Automatically generated ID.
    pub id: i32,

    /// Title to display in the task list.
    pub title: String,

    /// Description of the task.
    pub description: String,

    /// Whether the task is completed or not.
    pub completed: bool,

    /// The date the task was created.
    pub date_created: chrono::NaiveDateTime,

    /// The date the task was last modified.
    pub date_modified: Option<chrono::NaiveDateTime>,
}

/// Defines the data structure for a task summary.
#[derive(FromRow, Serialize)]
pub struct TaskSummary {
    /// Automatically generated ID.
    pub id: i32,

    /// Title to display in the task list.
    pub title: String,

    /// Whether the task is completed or not.
    pub completed: bool,

    /// The date the task was created.
    pub date_created: chrono::NaiveDateTime,

    /// The date the task was last modified.
    pub date_modified: Option<chrono::NaiveDateTime>,
}

/// Defines the data structure for a user.
#[derive(FromRow, Serialize)]
pub struct User {
    /// Automatically generated ID.
    pub id: i32,

    /// The email address associated with the user.
    pub email_address: String,

    /// The API key associated with the user.
    #[serde(skip_serializing)]
    pub api_key: String,

    /// The date the user information was created.
    pub date_created: chrono::NaiveDateTime,

    /// The date the user information was last modified.
    pub date_modified: Option<chrono::NaiveDateTime>,
}
