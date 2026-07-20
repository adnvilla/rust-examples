use serde_json_example::{read_order, write_order};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let order = read_order(r#"{"product":"té","quantity":2,"priority":true}"#)?;
    println!("Pedido tipado: {order:?}");
    println!("De vuelta a JSON: {}", write_order(&order)?);
    Ok(())
}
