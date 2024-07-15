use std::env;
use todo_api::{config::AppConfig, db, state::AppState, web};
use tokio::{net::TcpListener, signal};
use tracing::info;

/// The main function of the application.
///
/// This function is the entry point of the application. It loads the application configuration, connects to the database,
/// creates the application state, and starts the server.
///
/// Notice that we use the `#[tokio::main]` macro to mark this function as the main function of the application
/// instead of writing a regular entrypoint. This is because the logic in the API is asynchronous and wouldn't work
/// if we didn't instrument the main method with the [`tokio::main`] macro.
#[tokio::main]
async fn main() {
    init_tracing();

    let app_config = AppConfig::load().expect("Failed to load application configuration.");

    let connection_pool = db::connect_db(&app_config.database)
        .await
        .expect("Failed to connect to the database.");

    let app_state = AppState::new(connection_pool);
    let router = web::create_router(app_state);

    let listener = TcpListener::bind(app_config.server.to_address())
        .await
        .expect("Failed to bind to address.");

    info!(
        "Listening on {}:{}",
        app_config.server.host, app_config.server.port
    );

    // Make sure to use graceful shutdown on Ctrl+C otherwise the server will panic on shutdown.
    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            signal::ctrl_c()
                .await
                .expect("Failed to catch the SIGINT signal");
        })
        .await
        .unwrap();
}

/// This function initializes tracing so we can see logs from the application.
///
/// The logs are currently set up to show in the terminal. In a production scenario
/// you should consider setting up opentelemetry instead. There's a great package called
/// `tracing-opentelemetry` that can be used to send logs to a OTEL collector.
fn init_tracing() {
    use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};

    let rust_log = env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or_else(|_| "sqlx=info,tower_http=debug,info".to_string());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(&rust_log),
        )
        .init();
}
