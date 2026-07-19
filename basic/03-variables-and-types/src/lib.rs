#[must_use]
pub fn sell_tacos(stock: u16, requested: u16) -> (u16, u16) {
    let sold = stock.min(requested);
    (stock - sold, sold)
}

#[cfg(test)]
mod tests {
    use super::sell_tacos;

    #[test]
    fn never_sells_more_than_the_stock() {
        assert_eq!(sell_tacos(3, 5), (0, 3));
    }
}
