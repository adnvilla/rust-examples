use ferris_says::say;
use std::io::{self, BufWriter};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let message = "¡Hola, Rustaceans! Hoy el cangrejo trae dependencias.";
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());

    say(message, width, &mut writer)
}
