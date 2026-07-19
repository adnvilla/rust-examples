use enums_and_match::{DeliveryStatus, customer_message};

fn main() {
    let status = DeliveryStatus::OnTheWay { minutes: 8 };
    println!(
        "{}; el repartidor asegura que conoce un atajo.",
        customer_message(&status)
    );
}
