use collections_and_iterators::{best_seller, sales_totals};

fn main() {
    let totals = sales_totals([("café", 2), ("té", 4), ("café", 3), ("agua", 1)]);

    if let Some((product, quantity)) = best_seller(&totals) {
        println!("Ganador: {product} con {quantity} ventas. La cafetera exige una comisión.");
    }
}
