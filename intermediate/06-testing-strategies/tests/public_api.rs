use testing_strategies::apply_coupon;

#[test]
fn customer_pays_the_discounted_price() {
    assert_eq!(apply_coupon(2_500, 10), 2_250);
}
