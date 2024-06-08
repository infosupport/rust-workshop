use axum::Router;
use axum::routing::get;
use rest_api::configuration::ApplicationConfig;
use rest_api::database::connect_db;
use rest_api::endpoints::{task_list, task_details, create_task, delete_task, update_task};


async fn homepage() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let config = ApplicationConfig::load().expect("Failed to load configuration");
    let connection_pool = connect_db(config.database).await.expect("Failed to connect to database");
    let bind_address = format!("{}:{}", config.server.host, config.server.port);

    let router = Router::new()
        .route("/tasks/:id", get(task_details).patch(update_task).delete(delete_task))
        .route("/tasks", get(task_list).post(create_task));

    let listener = tokio::net::TcpListener::bind(bind_address)
        .await.expect("Can't bind to the specified host/port combination");

    axum::serve(listener, router).await.expect("Failed to start the server");
}
