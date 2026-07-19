use functions_and_expressions::calculate_tip;

fn main() {
    let subtotal = 2_500;
    let tip = calculate_tip(subtotal, true);
    println!("Subtotal: ${:.2}", f64::from(subtotal) / 100.0);
    println!(
        "Propina: ${:.2}; el mesero venció al café descafeinado.",
        f64::from(tip) / 100.0
    );
}
