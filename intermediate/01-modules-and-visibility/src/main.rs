use modules_and_visibility::{kitchen::Ticket, service::announce};

fn main() {
    let ticket = Ticket::new(4, "ramen");
    println!(
        "{} La cocina niega haber perdido los otros tres.",
        announce(&ticket)
    );
}
