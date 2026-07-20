use closures_and_iterators::{Sale, discounted_total};

fn main() {
    let sales = [
        Sale {
            product: "café",
            cents: 500,
        },
        Sale {
            product: "pastel",
            cents: 800,
        },
        Sale {
            product: "servilleta premium",
            cents: 50,
        },
    ];
    let total = discounted_total(&sales, 300, 10);
    println!("Total con descuento: ${:.2}", f64::from(total) / 100.0);
}
