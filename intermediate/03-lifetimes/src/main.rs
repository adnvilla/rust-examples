use lifetimes::{Review, most_detailed};

fn main() {
    let first = "Buen café";
    let second = "Buen café, silla sospechosa";
    let review = Review {
        author: "Ada",
        text: most_detailed(first, second),
    };
    println!("{}", review.summary());
}
