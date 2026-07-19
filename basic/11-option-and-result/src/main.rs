use option_and_result::parse_order;

fn main() {
    for input in ["2: café", "muchos croissants"] {
        match parse_order(input) {
            Ok((quantity, product)) => println!("Pedido aceptado: {quantity} × {product}"),
            Err(error) => {
                println!(
                    "No entendí «{input}»: {error}. Mi bola de cristal está en mantenimiento."
                );
            }
        }
    }
}
