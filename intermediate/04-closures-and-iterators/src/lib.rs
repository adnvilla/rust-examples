#[derive(Debug, PartialEq, Eq)]
pub struct Sale {
    pub product: &'static str,
    pub cents: u32,
}

#[must_use]
pub fn discounted_total(sales: &[Sale], minimum_cents: u32, discount_percent: u32) -> u32 {
    sales
        .iter()
        .filter(|sale| sale.cents >= minimum_cents)
        .map(|sale| sale.cents * (100 - discount_percent) / 100)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{Sale, discounted_total};

    #[test]
    fn pipeline_filters_maps_and_reduces() {
        let sales = [
            Sale {
                product: "café",
                cents: 500,
            },
            Sale {
                product: "galleta",
                cents: 200,
            },
        ];
        assert_eq!(discounted_total(&sales, 300, 10), 450);
    }
}
