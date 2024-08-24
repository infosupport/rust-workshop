use serde::Deserialize;

#[derive(Deserialize)]
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
#[derive(Deserialize)]
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
