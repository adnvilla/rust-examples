use traits_and_generics::{Card, Cash, receipt};

fn main() {
    println!(
        "{}",
        receipt(
            &Card {
                available_cents: 2_000
            },
            1_500
        )
    );
    println!(
        "{}; el billete prometió volver con amigos.",
        receipt(
            &Cash {
                received_cents: 1_000
            },
            1_500
        )
    );
}
