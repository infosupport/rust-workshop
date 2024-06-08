use serde::Serialize;
use crate::entity::Task;
use crate::errors::ValidationError;

#[derive(Serialize)]
pub struct TaskListResponse {
    pub items: Vec<Task>,
    pub page_index: i32,
    pub page_size: i32,
    pub total_count: i32,
}

#[derive(Serialize)]
pub struct CreateTaskResult {
    pub id: i32,
}

#[derive(Serialize)]
pub struct ErrorDetails {
    pub message: String,
}

#[derive(Serialize)]
pub struct ValidationErrorDetails {
    pub validation_errors: Vec<ValidationError>
}