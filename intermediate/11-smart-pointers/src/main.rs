use std::rc::Rc;

use smart_pointers::{Category, shared_owner_count};

fn main() {
    let category = Category::shared("postres");
    let sidebar = Rc::clone(&category);
    sidebar.visit();
    println!("{}", category.summary());
    println!(
        "Propietarios: {}; el pastel tiene representantes.",
        shared_owner_count(&category)
    );
}
