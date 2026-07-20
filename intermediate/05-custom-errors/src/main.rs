use custom_errors::parse_reservation;

fn main() {
    match parse_reservation("Ferris,30", 8) {
        Ok((name, size)) => println!("Mesa para {name}: {size} lugares"),
        Err(error) => println!("Reservación rechazada: {error}. Ferris invitó a todo el arrecife."),
    }
}
