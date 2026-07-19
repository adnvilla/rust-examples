use variables_and_types::sell_tacos;

fn main() {
    let taco_name: &str = "taco de pastor";
    let initial_stock: u16 = 12;
    let requested = 5;
    let (remaining, sold) = sell_tacos(initial_stock, requested);

    println!("Pedido: {sold} × {taco_name}");
    println!("Quedan {remaining} tacos; nadie culpará al turno nocturno.");
}
