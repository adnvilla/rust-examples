#[must_use]
pub fn label_length(label: &str) -> usize {
    label.chars().count()
}

pub fn mark_as_priority(label: &mut String) {
    label.push_str(" [PRIORIDAD]");
}

#[cfg(test)]
mod tests {
    use super::{label_length, mark_as_priority};

    #[test]
    fn reads_without_taking_ownership() {
        let label = String::from("Mesa 7");
        assert_eq!(label_length(&label), 6);
        assert_eq!(label, "Mesa 7");
    }

    #[test]
    fn changes_through_an_exclusive_borrow() {
        let mut label = String::from("Mesa 7");
        mark_as_priority(&mut label);
        assert_eq!(label, "Mesa 7 [PRIORIDAD]");
    }
}
