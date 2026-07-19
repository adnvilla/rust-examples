use channels::prepare_orders;

fn main() {
    for message in prepare_orders(vec!["ramen".to_owned(), "postre".to_owned()]) {
        println!("{message}");
    }
    println!("La cocina se comunicó sin un solo grito por el pasillo.");
}
