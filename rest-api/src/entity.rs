//! This module defines the data structures that represent the entities in the application.
//!
//! We mainly use these for loading/saving data in the database. You can also find these entities in the responses
//! of the web interface of the application.

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
}
