#[must_use]
pub fn add_delivery_note(mut order: String) -> String {
    order.push_str(" — entregar antes de que se enfríe");
    order
}

#[cfg(test)]
mod tests {
    use super::add_delivery_note;

    #[test]
    fn returns_the_transformed_owner() {
        let order = String::from("Dos cafés");
        assert_eq!(
            add_delivery_note(order),
            "Dos cafés — entregar antes de que se enfríe"
        );
    }
}
