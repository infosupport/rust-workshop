use std::env;
use todo_api::{config::AppConfig, db, state::AppState, web};
use tokio::{net::TcpListener, signal};
use tracing::info;

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
