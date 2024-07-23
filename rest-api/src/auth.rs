//! This module contains the authentication logic for the API.
//!
//! We're using the X-Api-Key header to authenticate users. The API key is used to identify the user and authorize
//! access to the API. This is not a very secure method of authentication, but it's simple and easy to implement.
//!
//! If you're looking for a more secure authentication method, you should consider using JWT tokens.
//! For an example of how to implement JWT authentication: https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs

use std::sync::Arc;

use crate::state::AppState;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{async_trait, Json};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use serde::Serialize;

use crate::db;

pub struct AuthenticatedUser {
    pub user_id: i32,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidApiKey,
}

#[derive(Serialize)]
struct ErrorDetails {
    message: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let response_data = match self {
            AuthError::InvalidApiKey => {
                let error_details = ErrorDetails {
                    message: "The requested user was not found.".to_string(),
                };

                (StatusCode::NOT_FOUND, Json(error_details))
            }
        };

        response_data.into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    SharedAppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = SharedAppState::from_ref(state);

        // Try to locate the API key in the headers collection.
        // If it's not there, or we can't parse the key, return an error.
        let api_key = parts
            .headers
            .get("X-Api-Key")
            .ok_or(AuthError::InvalidApiKey)?
            .to_str()
            .map_err(|_| AuthError::InvalidApiKey)?;

        let user = db::get_user_by_key(&state.connection_pool, api_key)
            .await
            .map_err(|_| AuthError::InvalidApiKey)?;

        // Return an authentication ticket for the user.
        Ok(AuthenticatedUser { user_id: user.id })
    }
}

type SharedAppState = Arc<AppState>;
