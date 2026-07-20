use zero_copy_parsing::parse_line;

fn main() {
    let input = String::from("WARN|la tostadora solicita vacaciones");
    let entry = parse_line(&input).expect("static example is valid");
    println!("[{}] {}", entry.level, entry.message);
}
