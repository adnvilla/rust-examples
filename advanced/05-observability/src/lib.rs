use tracing::{info, instrument, warn};

#[instrument(skip(product))]
pub fn prepare_order(order_id: u64, product: &str) -> bool {
    info!(%product, "preparing order");
    if product.trim().is_empty() {
        warn!("empty product rejected");
        return false;
    }
    info!("order ready");
    true
}

#[cfg(test)]
mod tests {
    use super::prepare_order;

    #[test]
    fn rejects_empty_products() {
        assert!(!prepare_order(7, ""));
    }
}
