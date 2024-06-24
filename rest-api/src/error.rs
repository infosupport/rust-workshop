//! This module contains the error handling logic for the application.
//!
//! ## Translating errors
//! The `AppError` enum is used to represent the different types of errors that can occur in the application.
//! We added various translation functions by implementing the [`core::convert::From<T>`] trait.
//!
//! You can see this conversion in action in the [`crate::config`] and [`crate::db`] modules.
//! Whenever we use the `?` operator to propagate an error, the error is translated to an [`crate::error::AppError`] if
//! there's an implementation of [`core::convert::From<T>`] for the error type that has a return type of [`crate::error::AppError`].
//!
//! ## Error responses in Axum
//! The handler functions in [`crate::web`] return a `Result<T, AppError>` type. When there's an error case for the result,
//! we want to convert this error case into a proper HTTP response.
//!
//! To convert the error into a HTTP response, we've implemented the [`IntoResponse`] trait for the [`crate::error::AppError`] enum.
//! Depending on the error you'll get a different status code and problem detail object.
use std::fmt;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

/// This alias is used to simplify the return type of functions that can return a [`crate::error::AppError`].
///
/// You'll see this often in rust applications because writing down the error type every time feels redundant.
/// It also ensures that we have a consistent error type across the application.
pub type Result<T, E = AppError> = std::result::Result<T, E>;

/// The different types of errors that can occur in the application.
///
/// Notice that there are error types that accept another error type as input. These errors are wrappers for library
/// errors. By wrapping library errors we ensure that we have a consistent interface without loosing the original error.
/// This is useful because we can add more context to the error and provide a better error message to the user.
///
/// By creating our own error enum we automatically gain insight into what errors you can expect from this application.
/// Making it easier to build proper error handling down the line.
#[derive(Debug)]
pub enum AppError {
    /// When the application is misconfigured you'll get this error. The details of this error will tell you what is
    /// wrong. Most commonly this error is thrown when the `config.toml` file is missing keys or has the wrong format.
    ConfigError(config::ConfigError),

    /// When the application can't communicate with the database, this error is returned. The details explain exactly
    /// what went wrong. The most common error is a connection error so make sure to check if your database server
    /// is running.
    DbError(sqlx::Error),

    /// When a TODO item can't be found, this error is returned. This error isn't fixable by the user and is used to
    /// indicate that the requested TODO item doesn't exist. The error is automatically translated to a 404.
    TodoNotFound,
}

/// The details of an error that are shown to the application user.
#[derive(Serialize)]
struct ErrorDetails {
    /// The error message shown to the application user.
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigError(_) => write!(
                f,
                "Configuration error. Please check your configuration file."
            ),
            AppError::DbError(_) => {
                write!(f, "An error occurred while interacting with the database.")
            }
            AppError::TodoNotFound => write!(f, "The requested todo item was not found."),
        }
    }
}

impl From<config::ConfigError> for AppError {
    fn from(value: config::ConfigError) -> Self {
        AppError::ConfigError(value)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::DbError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let response_data = match self {
            AppError::TodoNotFound => {
                let error_details = ErrorDetails {
                    message: "The requested todo item was not found.".to_string(),
                };

                (StatusCode::NOT_FOUND, Json(error_details))
            }
            AppError::ConfigError(_) | AppError::DbError(_) => {
                let error_details = ErrorDetails {
                    message: "Internal server error".to_string(),
                };

                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_details))
            }
        };

        response_data.into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_config_error_returns_correct_error_type() {
        let err = config::ConfigError::NotFound("test".to_string());
        let app_err = AppError::from(err);

        match app_err {
            AppError::ConfigError(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
