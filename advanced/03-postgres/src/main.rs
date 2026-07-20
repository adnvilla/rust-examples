use postgres_example::database_url;
use sqlx::{PgPool, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPool::connect(&database_url()).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS orders (id BIGSERIAL PRIMARY KEY, product TEXT NOT NULL)",
    )
    .execute(&pool)
    .await?;
    sqlx::query("INSERT INTO orders (product) VALUES ($1)")
        .bind("café efímero")
        .execute(&pool)
        .await?;
    let count: i64 = sqlx::query("SELECT COUNT(*) AS count FROM orders")
        .fetch_one(&pool)
        .await?
        .get("count");
    println!("Pedidos persistidos: {count}");
    Ok(())
}
