pub mod kitchen {
    #[derive(Debug, PartialEq, Eq)]
    pub struct Ticket {
        table: u16,
        dish: String,
    }

    impl Ticket {
        #[must_use]
        pub fn new(table: u16, dish: impl Into<String>) -> Self {
            Self {
                table,
                dish: dish.into(),
            }
        }

        #[must_use]
        pub fn label(&self) -> String {
            format!("Mesa {}: {}", self.table, self.dish)
        }
    }
}

pub mod service {
    use crate::kitchen::Ticket;

    #[must_use]
    pub fn announce(ticket: &Ticket) -> String {
        format!("¡Sale {}!", ticket.label())
    }
}

#[cfg(test)]
mod tests {
    use super::{kitchen::Ticket, service::announce};

    #[test]
    fn modules_collaborate_through_public_apis() {
        assert_eq!(announce(&Ticket::new(4, "ramen")), "¡Sale Mesa 4: ramen!");
    }
}
