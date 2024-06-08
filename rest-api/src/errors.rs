use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::responses::{ErrorDetails, ValidationErrorDetails};

pub enum AppError {
    TaskNotFound,
    ValidationFailed {
        errors: Vec<ValidationError>
    },
}

#[derive(Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::TaskNotFound => {
                let error_details = ErrorDetails { message: "Task not found".to_string() };
                (StatusCode::NOT_FOUND, Json(error_details)).into_response()
            }
            AppError::ValidationFailed { errors } => {
                let error_details = ValidationErrorDetails {
                    validation_errors: errors
                };

                (StatusCode::BAD_REQUEST, Json(error_details)).into_response()
            }
        }
    }
}

impl Into<AppError> for Vec<ValidationError> {
    fn into(self) -> AppError {
        AppError::ValidationFailed { errors: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    #[test]
    fn task_not_found_error_returns_not_found_status() {
        let error = AppError::TaskNotFound;
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn validation_failed_error_returns_bad_request_status() {
        let error = AppError::ValidationFailed { errors: vec![] };
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn validation_failed_error_includes_all_errors_in_response() {
        let validation_errors = vec![
            ValidationError { field: "field1".to_string(), message: "error1".to_string() },
            ValidationError { field: "field2".to_string(), message: "error2".to_string() },
        ];

        let error = AppError::ValidationFailed { errors: validation_errors };

        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_into() {
        let errors = vec![
            ValidationError { field: "field1".to_string(), message: "error1".to_string() },
            ValidationError { field: "field2".to_string(), message: "error2".to_string() },
        ];

        let app_error: AppError = errors.into();

        assert!(matches!(app_error, AppError::ValidationFailed { .. }));
    }
}