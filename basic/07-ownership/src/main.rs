use ownership::add_delivery_note;

fn main() {
    let order = String::from("Dos cafés");
    let order = add_delivery_note(order);
    println!("{order}");
}
