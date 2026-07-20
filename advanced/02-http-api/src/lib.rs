use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct AppState {
    orders: Arc<RwLock<Vec<Order>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Order {
    pub product: String,
    pub quantity: u16,
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/orders", get(list_orders).post(create_order))
        .with_state(state)
}

async fn list_orders(State(state): State<AppState>) -> Json<Vec<Order>> {
    Json(state.orders.read().await.clone())
}

async fn create_order(State(state): State<AppState>, Json(order): Json<Order>) -> Json<Order> {
    state.orders.write().await.push(order.clone());
    Json(order)
}

#[cfg(test)]
mod tests {
    use super::Order;

    #[test]
    fn order_is_a_typed_contract() {
        assert_eq!(
            Order {
                product: "té".to_owned(),
                quantity: 2
            }
            .quantity,
            2
        );
    }
}
