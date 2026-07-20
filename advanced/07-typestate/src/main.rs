use typestate::Order;

fn main() {
    let shipped = Order::new("teclado mecánico silencioso").pay().ship();
    println!("{}; esta vez sí era silencioso.", shipped.confirmation());
}
