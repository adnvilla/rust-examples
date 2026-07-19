use strings_and_unicode::{describe_text, make_greeting};

fn main() {
    let guest = "Zoë";
    let greeting = make_greeting(guest);
    let (bytes, characters) = describe_text(&greeting);

    println!("{greeting}");
    println!("Ocupa {bytes} bytes y contiene {characters} caracteres.");
}
