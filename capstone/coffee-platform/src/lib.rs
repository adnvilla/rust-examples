use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use tracing::{info, instrument, warn};

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    cache: redis::Client,
}

impl AppState {
    #[must_use]
    pub fn new(pool: PgPool, cache: redis::Client) -> Self {
        Self { pool, cache }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewOrder {
    pub product: String,
    pub quantity: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Order {
    pub id: i64,
    pub product: String,
    pub quantity: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    EmptyProduct,
    InvalidQuantity,
}

/// Comprueba reglas que no requieren infraestructura.
///
/// # Errors
///
/// Devuelve [`ValidationError`] para producto vacío o cantidad cero.
pub fn validate_order(order: &NewOrder) -> Result<(), ValidationError> {
    if order.product.trim().is_empty() {
        return Err(ValidationError::EmptyProduct);
    }
    if order.quantity == 0 {
        return Err(ValidationError::InvalidQuantity);
    }
    Ok(())
}

#[derive(Debug)]
enum ApiError {
    Validation(ValidationError),
    Database(sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Validation(error) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("pedido inválido: {error:?}"),
            )
                .into_response(),
            Self::Database(error) => {
                tracing::error!(%error, "database operation failed");
                (StatusCode::INTERNAL_SERVER_ERROR, "error de persistencia").into_response()
            }
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/orders", get(list_orders).post(create_order))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> StatusCode {
    if sqlx::query("SELECT 1").execute(&state.pool).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

#[instrument(skip(state, input), fields(product = %input.product))]
async fn create_order(
    State(state): State<AppState>,
    Json(input): Json<NewOrder>,
) -> Result<(StatusCode, Json<Order>), ApiError> {
    validate_order(&input).map_err(ApiError::Validation)?;
    let row = sqlx::query(
        "INSERT INTO orders (product, quantity) VALUES ($1, $2) RETURNING id, product, quantity",
    )
    .bind(input.product.trim())
    .bind(i32::from(input.quantity))
    .fetch_one(&state.pool)
    .await?;
    invalidate_cache(&state.cache).await;
    let order = row_to_order(&row);
    info!(order_id = order.id, "order created");
    Ok((StatusCode::CREATED, Json(order)))
}

#[instrument(skip(state))]
async fn list_orders(State(state): State<AppState>) -> Result<Json<Vec<Order>>, ApiError> {
    if let Some(orders) = read_cache(&state.cache).await {
        info!("cache hit");
        return Ok(Json(orders));
    }
    info!("cache miss");
    let rows = sqlx::query("SELECT id, product, quantity FROM orders ORDER BY id")
        .fetch_all(&state.pool)
        .await?;
    let orders: Vec<_> = rows.iter().map(row_to_order).collect();
    write_cache(&state.cache, &orders).await;
    Ok(Json(orders))
}

fn row_to_order(row: &sqlx::postgres::PgRow) -> Order {
    Order {
        id: row.get("id"),
        product: row.get("product"),
        quantity: row.get("quantity"),
    }
}

async fn read_cache(client: &redis::Client) -> Option<Vec<Order>> {
    let mut connection = client.get_multiplexed_async_connection().await.ok()?;
    let json: Option<String> = connection.get("orders:v1").await.ok()?;
    serde_json::from_str(&json?).ok()
}

async fn write_cache(client: &redis::Client, orders: &[Order]) {
    let Ok(json) = serde_json::to_string(orders) else {
        return;
    };
    let Ok(mut connection) = client.get_multiplexed_async_connection().await else {
        warn!("cache unavailable while writing");
        return;
    };
    if let Err(error) = connection.set_ex::<_, _, ()>("orders:v1", json, 30).await {
        warn!(%error, "cache write failed");
    }
}

async fn invalidate_cache(client: &redis::Client) {
    if let Ok(mut connection) = client.get_multiplexed_async_connection().await {
        let _: redis::RedisResult<()> = connection.del("orders:v1").await;
    }
}

/// Crea el esquema mínimo usado por el ejemplo.
///
/// # Errors
///
/// Propaga errores de conexión o ejecución reportados por `SQLx`.
pub async fn prepare_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS orders (id BIGSERIAL PRIMARY KEY, product TEXT NOT NULL, quantity INTEGER NOT NULL CHECK (quantity > 0))",
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{NewOrder, ValidationError, validate_order};

    #[test]
    fn rejects_orders_that_should_not_reach_infrastructure() {
        assert_eq!(
            validate_order(&NewOrder {
                product: String::new(),
                quantity: 2
            }),
            Err(ValidationError::EmptyProduct)
        );
        assert_eq!(
            validate_order(&NewOrder {
                product: "café".to_owned(),
                quantity: 0
            }),
            Err(ValidationError::InvalidQuantity)
        );
    }
}
