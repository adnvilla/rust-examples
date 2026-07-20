use redis::{AsyncCommands, SetExpiry, SetOptions};
use redis_cache::redis_url;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open(redis_url())?;
    let mut connection = client.get_multiplexed_async_connection().await?;
    let options = SetOptions::default().with_expiration(SetExpiry::EX(30));
    let _: () = connection
        .set_options("menu:featured", "café", options)
        .await?;
    let featured: Option<String> = connection.get("menu:featured").await?;
    println!("Caché: {featured:?}; en 30 segundos olvidará con elegancia.");
    Ok(())
}
