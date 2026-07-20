/// Aplica un cupón porcentual a un precio en centavos.
///
/// ```
/// use testing_strategies::apply_coupon;
/// assert_eq!(apply_coupon(1_000, 20), 800);
/// ```
#[must_use]
pub fn apply_coupon(price_cents: u32, percent: u8) -> u32 {
    let safe_percent = percent.min(100);
    price_cents * (100 - u32::from(safe_percent)) / 100
}

#[cfg(test)]
mod tests {
    use super::apply_coupon;

    #[test]
    fn caps_overenthusiastic_coupons() {
        assert_eq!(apply_coupon(500, 200), 0);
    }
}
