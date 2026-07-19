#[must_use]
pub fn calculate_tip(subtotal_cents: u32, service_was_great: bool) -> u32 {
    let percentage = if service_was_great { 20 } else { 10 };
    subtotal_cents * percentage / 100
}

#[cfg(test)]
mod tests {
    use super::calculate_tip;

    #[test]
    fn rewards_great_service() {
        assert_eq!(calculate_tip(2_500, true), 500);
    }
}
