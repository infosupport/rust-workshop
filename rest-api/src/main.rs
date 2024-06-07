use axum::Router;
use axum::routing::get;
use rest_api::configuration::ApplicationConfig;
use rest_api::database::connect_db;

mod configuration;

async fn homepage() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let config = ApplicationConfig::load().expect("Failed to load configuration");
    let connection_pool = connect_db(config.database).await.expect("Failed to connect to database");
    let bind_address = format!("{}:{}", config.server.host.to_owned(), config.server.port.to_owned());

    let router = Router::new().route("/", get(homepage));

    let listener = tokio::net::TcpListener::bind(bind_address)
        .await.expect("Can't bind to the specified host/port combination");

    axum::serve(listener, router).await.expect("Failed to start the server");
}
