use std::collections::HashMap;

#[must_use]
pub fn sales_totals<'a>(sales: impl IntoIterator<Item = (&'a str, u32)>) -> HashMap<&'a str, u32> {
    let mut totals = HashMap::new();
    for (product, quantity) in sales {
        *totals.entry(product).or_insert(0) += quantity;
    }
    totals
}

#[must_use]
// Esta lección usa deliberadamente el hasher estándar; los hashers genéricos
// aparecen después de introducir traits en el nivel intermedio.
#[allow(clippy::implicit_hasher)]
pub fn best_seller<'a>(totals: &'a HashMap<&'a str, u32>) -> Option<(&'a str, u32)> {
    totals
        .iter()
        .max_by_key(|(product, quantity)| (*quantity, *product))
        .map(|(product, quantity)| (*product, *quantity))
}

#[cfg(test)]
mod tests {
    use super::{best_seller, sales_totals};

    #[test]
    fn groups_sales_and_finds_the_winner() {
        let totals = sales_totals([("café", 2), ("té", 1), ("café", 3)]);
        assert_eq!(totals.get("café"), Some(&5));
        assert_eq!(best_seller(&totals), Some(("café", 5)));
    }
}
