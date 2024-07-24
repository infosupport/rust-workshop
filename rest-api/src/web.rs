//! This module contains the router and associated handlers for the web application.
//!
//! The router is an [`axum::Router`] object that is configured with various routes.
//! Each route is associated with a handler function that is called when the route is matched.
//!
//! A route handler is implemented as an async function that returns [`crate::error::Result`] type.
//! Both cases of the result type need to be translatable into an HTTP response. In other words, they need to
//! implement the [`IntoResponse`] trait.
//!
//! For the error case of [`Result`] we use the [`AppError`] type. This type is translatable into an HTTP response
//! with the [`IntoResponse`] trait. Please check out the [`crate::error`] module for more information.
//!
//! Handlers need access to various pieces of information in the request to perform work. This information is passed
//! using extractors. You can find more information about extractors in the [`axum::extract`] module.
//!
//! To access the database, the handlers need access to the application state. The application state is a shared
//! object that is obtained using the [`axum::extract::State`] extractor. The application state is a shared object
//! that is created in the [`crate::state`] module. We use the [`Arc`] type to share the state across multiple threads.

use std::sync::Arc;

use crate::entity::ApiKey;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::instrument;

use crate::{auth::AuthenticatedUser, db, error::AppError, state::AppState};

/// Defines the querystring parameters for retrieving todos.
#[derive(Deserialize, Debug)]
struct Pagination {
    page: i32,
}

/// Defines the fields that can be used to create a new todo item.
#[derive(Deserialize, Debug)]
struct CreateTodoForm {
    pub title: String,
    pub description: String,
}

/// Defines the fields that can be updated in a todo item.
#[derive(Deserialize, Debug)]
struct UpdateTodoForm {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

/// Defines the fields that can be used to register a new user.
#[derive(Deserialize, Debug)]
struct RegisterUserForm {
    pub email_address: String,
}

/// Defines the response structure for a user that has been created.
#[derive(Serialize, Debug)]
struct UserCreatedResponse {
    /// The generated API key for the user.
    pub api_key: String,
}

/// Retrieves a list of todos from the database and renders them as a JSON response.
///
/// The URL must include `?page=<number>` to specify which page to include. The page parameter is retrieved using the
/// [`Query`] extractor. If you want to control the page_size as well, you should add the field for it to the [`Pagination`]
/// struct and update the call to the [`db::list_tasks`] function.
///
/// This function uses the [`State`] extractor to obtain the shared application state. The application state contains the
/// database connection pool that is used to retrieve the todo items.
///
/// In addition to the shared state, we also use the [`AuthenticatedUser`] extractor to obtain the user
/// ID of the authenticated user. If this extractor fails, we automatically return a 401 Unauthorized response.
#[instrument]
async fn list_todos(
    State(app_state): State<Arc<AppState>>,
    Query(pagination): Query<Pagination>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let result = db::list_tasks(&app_state.connection_pool, user_id, pagination.page, 10).await?;
    Ok(Json(result))
}

/// Retrieves a single todo with details of the todo item.
///
/// The URL includes a dynamic segment `:id` (see the [`create_router`] implementation for the details). The `:id` segment
/// is mapped using the [`Path`] extractor. The `id` is then used to retrieve the todo item from the database.
///
/// This function uses the [`State`] extractor to obtain the shared application state. The application state contains the
/// database connection pool that is used to retrieve the todo item.
#[instrument]
async fn todo_details(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let result = db::find_task(&app_state.connection_pool, user_id, id)
        .await
        .map(|todo| Json(todo))?;

    Ok(result)
}

/// Creates a new todo item in the database.
///
/// The request body must be a JSON object that can be deserialized to [`CreateTodoForm`].
/// We're using [`serde`] to deserialize the JSON object into a [`CreateTodoForm`] struct.
///
/// This function uses the [`State`] extractor to obtain the shared application state. The application state contains the
/// database connection pool that is used to retrieve the todo item.
#[instrument]
async fn create_todo(
    State(app_state): State<Arc<AppState>>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(form): Json<CreateTodoForm>,
) -> Result<impl IntoResponse, AppError> {
    db::insert_task(
        &app_state.connection_pool,
        user_id,
        form.title.clone(),
        form.description.clone(),
    )
    .await?;

    Ok((StatusCode::CREATED, ()))
}

/// Updates an existing todo item in the database.
///
/// The URL includes a dynamic segment `:id` (see the [`create_router`] implementation for the details). The `:id` segment
/// is mapped using the [`Path`] extractor. The `id` is then used to retrieve the todo item from the database.
///
/// The request body must be a JSON object that can be deserialized to [`CreateTodoForm`].
/// We're using [`serde`] to deserialize the JSON object into a [`CreateTodoForm`] struct.
///
/// This function uses the [`State`] extractor to obtain the shared application state. The application state contains the
/// database connection pool that is used to retrieve the todo item.
#[instrument]
async fn update_todo(
    State(app_state): State<Arc<AppState>>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(form): Json<UpdateTodoForm>,
) -> Result<impl IntoResponse, AppError> {
    db::update_task(
        &app_state.connection_pool,
        user_id,
        id,
        form.title.clone(),
        form.description.clone(),
        form.completed,
    )
    .await?;

    Ok((StatusCode::ACCEPTED, ()))
}

/// Removes a todo item from the database.
///
/// The URL includes a dynamic segment `:id` (see the [`create_router`] implementation for the details). The `:id` segment
/// is mapped using the [`Path`] extractor. The `id` is then used to retrieve the todo item from the database.
///
/// This function uses the [`State`] extractor to obtain the shared application state. The application state contains the
/// database connection pool that is used to retrieve the todo item.
#[instrument]
async fn delete_todo(
    State(app_state): State<Arc<AppState>>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    db::delete_task(&app_state.connection_pool, user_id, id).await?;
    Ok((StatusCode::NO_CONTENT, ()))
}

/// Register a new user with associated API key.
#[instrument]
async fn register_user(
    State(app_state): State<Arc<AppState>>,
    Json(form): Json<RegisterUserForm>,
) -> Result<impl IntoResponse, AppError> {
    // Generate a random hex string 24 characters long.
    let api_key = ApiKey::new();

    db::insert_user(
        &app_state.connection_pool,
        form.email_address.clone(),
        api_key.hash.clone(),
    )
    .await?;

    // Returns the API Key for the user. This is a sensitive piece of information and should be handled with care.
    // We're returning it here for the user to write it down. It will be gone afterwards.
    Ok((
        StatusCode::CREATED,
        Json(UserCreatedResponse {
            api_key: api_key.key,
        }),
    ))
}

/// Creates the router for the web application.
///
/// The router configures various routes to assiocated handler functions. Each handler function can use the
/// application state thanks to the [`axum::Router::with_state`] method call that we've added. Although we use one
/// piece of state information you're free to add more if needed.
#[instrument]
pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/v1/todos/:id",
            get(todo_details).put(update_todo).delete(delete_todo),
        )
        .route("/v1/todos", get(list_todos).post(create_todo))
        .route("/v1/users/register", post(register_user))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}
