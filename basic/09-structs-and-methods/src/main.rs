use structs_and_methods::CoffeeOrder;

fn main() {
    let order = CoffeeOrder::new("Linus", 2, false);
    println!("{}; compilar kernels también da sed.", order.summary());
}
