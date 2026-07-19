#[derive(Debug, PartialEq, Eq)]
pub struct CoffeeOrder {
    customer: String,
    shots: u8,
    iced: bool,
}

impl CoffeeOrder {
    pub fn new(customer: impl Into<String>, shots: u8, iced: bool) -> Self {
        Self {
            customer: customer.into(),
            shots,
            iced,
        }
    }

    #[must_use]
    pub fn summary(&self) -> String {
        let temperature = if self.iced { "frío" } else { "caliente" };
        format!(
            "{}: café {temperature} con {} shot(s)",
            self.customer, self.shots
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CoffeeOrder;

    #[test]
    fn summarizes_a_typed_order() {
        let order = CoffeeOrder::new("Linus", 2, false);
        assert_eq!(order.summary(), "Linus: café caliente con 2 shot(s)");
    }
}
