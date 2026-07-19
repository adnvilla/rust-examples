#[derive(Debug, PartialEq, Eq)]
pub enum DeliveryStatus {
    Preparing,
    OnTheWay { minutes: u16 },
    Delivered { receiver: String },
    Cancelled(String),
}

#[must_use]
pub fn customer_message(status: &DeliveryStatus) -> String {
    match status {
        DeliveryStatus::Preparing => "Estamos preparando tu pedido".to_owned(),
        DeliveryStatus::OnTheWay { minutes } => format!("Llega en {minutes} minutos"),
        DeliveryStatus::Delivered { receiver } => format!("Entregado a {receiver}"),
        DeliveryStatus::Cancelled(reason) => format!("Cancelado: {reason}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{DeliveryStatus, customer_message};

    #[test]
    fn extracts_data_from_the_active_variant() {
        let status = DeliveryStatus::OnTheWay { minutes: 8 };
        assert_eq!(customer_message(&status), "Llega en 8 minutos");
    }
}
