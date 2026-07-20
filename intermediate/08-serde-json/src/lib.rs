use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Order {
    pub product: String,
    pub quantity: u16,
    #[serde(default)]
    pub priority: bool,
}

/// Deserializa y valida un pedido JSON.
///
/// # Errors
///
/// Devuelve un error de Serde si el JSON o sus tipos no cumplen el contrato.
pub fn read_order(json: &str) -> Result<Order, serde_json::Error> {
    serde_json::from_str(json)
}

/// Serializa un pedido como JSON compacto.
///
/// # Errors
///
/// Devuelve un error de Serde si el valor no puede serializarse.
pub fn write_order(order: &Order) -> Result<String, serde_json::Error> {
    serde_json::to_string(order)
}

#[cfg(test)]
mod tests {
    use super::{Order, read_order, write_order};

    #[test]
    fn round_trip_preserves_the_order() {
        let order = read_order(r#"{"product":"café","quantity":2}"#).expect("valid fixture");
        assert_eq!(
            order,
            Order {
                product: "café".to_owned(),
                quantity: 2,
                priority: false
            }
        );
        let json = write_order(&order).expect("serializable");
        assert_eq!(read_order(&json).expect("valid round trip"), order);
    }
}
