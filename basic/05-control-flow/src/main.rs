use control_flow::first_available_slot;

fn main() {
    let slots = [true, true, false, true];

    match first_available_slot(&slots) {
        Some(number) => println!("Lugar {number} disponible. El robot deja de dar vueltas."),
        None => println!("Estacionamiento lleno. El robot reconsidera sus decisiones."),
    }
}
