use std::thread;

/// Suma dos mitades del slice mediante threads acotados.
///
/// # Panics
///
/// Entra en pánico si alguno de los workers entra en pánico antes de terminar.
#[must_use]
pub fn parallel_sum(values: &[u64]) -> u64 {
    if values.is_empty() {
        return 0;
    }
    let middle = values.len() / 2;
    let (left, right) = values.split_at(middle);

    thread::scope(|scope| {
        let left_worker = scope.spawn(|| left.iter().sum::<u64>());
        let right_worker = scope.spawn(|| right.iter().sum::<u64>());
        left_worker.join().expect("left worker panicked")
            + right_worker.join().expect("right worker panicked")
    })
}

#[cfg(test)]
mod tests {
    use super::parallel_sum;

    #[test]
    fn scoped_threads_borrow_the_input_safely() {
        assert_eq!(parallel_sum(&[10, 20, 30, 40]), 100);
        assert_eq!(parallel_sum(&[]), 0);
    }
}
