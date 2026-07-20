use std::marker::PhantomData;

pub struct Draft;
pub struct Paid;
pub struct Shipped;

pub struct Order<State> {
    product: String,
    _state: PhantomData<State>,
}

impl Order<Draft> {
    #[must_use]
    pub fn new(product: impl Into<String>) -> Self {
        Self {
            product: product.into(),
            _state: PhantomData,
        }
    }

    #[must_use]
    pub fn pay(self) -> Order<Paid> {
        Order {
            product: self.product,
            _state: PhantomData,
        }
    }
}

impl Order<Paid> {
    #[must_use]
    pub fn ship(self) -> Order<Shipped> {
        Order {
            product: self.product,
            _state: PhantomData,
        }
    }
}

impl Order<Shipped> {
    #[must_use]
    pub fn confirmation(&self) -> String {
        format!("{} fue enviado", self.product)
    }
}

#[cfg(test)]
mod tests {
    use super::Order;

    #[test]
    fn valid_transitions_consume_the_previous_state() {
        assert_eq!(
            Order::new("libro").pay().ship().confirmation(),
            "libro fue enviado"
        );
    }
}
