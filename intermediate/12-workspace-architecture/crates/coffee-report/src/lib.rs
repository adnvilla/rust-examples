use coffee_domain::Sale;

#[must_use]
pub fn daily_report(sales: &[Sale]) -> String {
    let total: u32 = sales.iter().map(|sale| sale.cents).sum();
    format!(
        "{} venta(s), total: ${:.2}",
        sales.len(),
        f64::from(total) / 100.0
    )
}

#[cfg(test)]
mod tests {
    use coffee_domain::Sale;

    use super::daily_report;

    #[test]
    fn reports_domain_values_through_a_public_dependency() {
        assert_eq!(
            daily_report(&[Sale::new("café", 500)]),
            "1 venta(s), total: $5.00"
        );
    }
}
