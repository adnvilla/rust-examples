#[derive(Debug, PartialEq, Eq)]
pub struct Sale {
    pub product: String,
    pub cents: u32,
}

impl Sale {
    #[must_use]
    pub fn new(product: impl Into<String>, cents: u32) -> Self {
        Self {
            product: product.into(),
            cents,
        }
    }
}
