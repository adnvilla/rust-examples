use http_api::{AppState, app};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("API en http://127.0.0.1:3000; los cafés ya tienen endpoint.");
    axum::serve(listener, app(AppState::default())).await
}
