use coffee_platform::{AppState, app, prepare_database};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://rust:rust@127.0.0.1:5432/coffee_platform".to_owned());
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_owned());
    let address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_owned());
    let pool = PgPool::connect(&database_url).await?;
    prepare_database(&pool).await?;
    let cache = redis::Client::open(redis_url)?;
    let listener = TcpListener::bind(&address).await?;
    tracing::info!(%address, "coffee platform listening");
    axum::serve(listener, app(AppState::new(pool, cache)))
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
