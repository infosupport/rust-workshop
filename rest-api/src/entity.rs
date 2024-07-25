//! This module defines the data structures that represent the entities in the application.
//!
//! We mainly use these for loading/saving data in the database. You can also find these entities in the responses
//! of the web interface of the application.
//!
//! Note that not all fields are serialized by the API. For example, the generated API key for a user is not serialized.

use rand::{distributions::Alphanumeric, thread_rng, Rng};
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

/// Represents an API key in its original form and its hashed form.
/// The original key is only available when the user submits the key or when the key is created.
/// The hashed key is used to compare the key with the one stored in the database.
pub struct ApiKey {
    pub key: String,
    pub hash: String,
}

impl ApiKey {
    /// Generate a new API key from a random string.
    pub fn new() -> Self {
        let key: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let hash = sha256::digest(key.clone());

        Self { key, hash }
    }

    /// Create an API key from a string.
    pub fn from_string(key: &String) -> Self {
        let hash = sha256::digest(key.clone());

        Self {
            key: key.clone(),
            hash,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_api_key_new() {
        let key = super::ApiKey::new();
        assert_eq!(key.key.len(), 30);
    }

    #[test]
    fn test_api_key_from_string() {
        let key = "test".to_string();
        let api_key = super::ApiKey::from_string(&key);
        assert_eq!(api_key.key, key);
        assert_eq!(
            api_key.hash,
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        )
    }
}
