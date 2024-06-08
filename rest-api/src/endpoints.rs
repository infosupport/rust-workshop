use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::entity::Task;
use crate::errors::AppError;
use crate::requests::CreateTaskForm;
use crate::responses::{CreateTaskResult, TaskListResponse};

pub async fn task_list() -> Result<impl IntoResponse, AppError> {
    let task_list = TaskListResponse { items: vec![], page_index: 0, page_size: 0, total_count: 0 };
    Ok(Json(task_list))
}

pub async fn task_details(Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let task_details = Task { id, description: "Do the laundry".to_string(), completed: false };
    Ok(Json(task_details))
}

pub async fn create_task(Json(form_data): Json<CreateTaskForm>) -> Result<impl IntoResponse, AppError> {
    let validation_errors = form_data.validate();

    if validation_errors.len() > 0 {
        return Err(validation_errors.into())
    }

    Ok((StatusCode::CREATED, Json(CreateTaskResult { id: 1 })))
}

pub async fn delete_task(Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_task(Path(id): Path<i32>, Json(_form_data): Json<CreateTaskForm>) -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::ACCEPTED, ()))
}

