use borrowing::{label_length, mark_as_priority};

fn main() {
    let mut label = String::from("Mesa 7");
    println!("La etiqueta tiene {} caracteres.", label_length(&label));
    mark_as_priority(&mut label);
    println!("{label}: alguien pidió café antes de hablarle.");
}
