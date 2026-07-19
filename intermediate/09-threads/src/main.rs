use threads::parallel_sum;

fn main() {
    let orders = [12, 8, 15, 5, 20, 10];
    println!(
        "Total procesado en dos hilos: {} pedidos",
        parallel_sum(&orders)
    );
}
