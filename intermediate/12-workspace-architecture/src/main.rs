use coffee_domain::Sale;
use coffee_report::daily_report;

fn main() {
    let sales = [Sale::new("café", 500), Sale::new("pastel", 800)];
    println!("{}", daily_report(&sales));
}
