pub trait PaymentMethod {
    fn label(&self) -> &'static str;
    fn authorize(&self, cents: u32) -> bool;
}

pub struct Card {
    pub available_cents: u32,
}

pub struct Cash {
    pub received_cents: u32,
}

impl PaymentMethod for Card {
    fn label(&self) -> &'static str {
        "tarjeta"
    }

    fn authorize(&self, cents: u32) -> bool {
        self.available_cents >= cents
    }
}

impl PaymentMethod for Cash {
    fn label(&self) -> &'static str {
        "efectivo"
    }

    fn authorize(&self, cents: u32) -> bool {
        self.received_cents >= cents
    }
}

#[must_use]
pub fn receipt<M: PaymentMethod>(method: &M, cents: u32) -> String {
    let status = if method.authorize(cents) {
        "aprobado"
    } else {
        "rechazado"
    };
    format!("Pago con {}: {status}", method.label())
}

#[cfg(test)]
mod tests {
    use super::{Card, receipt};

    #[test]
    fn generic_code_uses_the_trait_contract() {
        assert_eq!(
            receipt(
                &Card {
                    available_cents: 900
                },
                500
            ),
            "Pago con tarjeta: aprobado"
        );
    }
}
