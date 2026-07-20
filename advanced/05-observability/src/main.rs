use observability::prepare_order;

fn main() {
    tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .init();
    prepare_order(42, "espresso");
}
