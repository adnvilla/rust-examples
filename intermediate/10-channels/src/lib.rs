use std::{sync::mpsc, thread};

/// Prepara pedidos en un worker y recolecta sus mensajes por un channel.
///
/// # Panics
///
/// Entra en pánico si el worker de cocina entra en pánico.
#[must_use]
pub fn prepare_orders(orders: Vec<String>) -> Vec<String> {
    let (sender, receiver) = mpsc::channel();
    let worker = thread::spawn(move || {
        for order in orders {
            if sender.send(format!("listo: {order}")).is_err() {
                break;
            }
        }
    });

    let prepared: Vec<_> = receiver.into_iter().collect();
    worker.join().expect("kitchen worker panicked");
    prepared
}

#[cfg(test)]
mod tests {
    use super::prepare_orders;

    #[test]
    fn transfers_owned_messages_between_threads() {
        assert_eq!(
            prepare_orders(vec!["café".to_owned(), "té".to_owned()]),
            ["listo: café", "listo: té"]
        );
    }
}
