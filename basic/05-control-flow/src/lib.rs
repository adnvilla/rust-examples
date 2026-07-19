#[must_use]
pub fn first_available_slot(slots: &[bool]) -> Option<usize> {
    for (position, occupied) in slots.iter().enumerate() {
        if !occupied {
            return Some(position + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::first_available_slot;

    #[test]
    fn finds_the_first_free_slot() {
        assert_eq!(first_available_slot(&[true, true, false, false]), Some(3));
    }

    #[test]
    fn reports_a_full_parking_lot() {
        assert_eq!(first_available_slot(&[true, true]), None);
    }
}
