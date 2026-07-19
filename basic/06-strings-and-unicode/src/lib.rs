#[must_use]
pub fn describe_text(text: &str) -> (usize, usize) {
    (text.len(), text.chars().count())
}

#[must_use]
pub fn make_greeting(name: &str) -> String {
    format!("¡Hola, {name}! ☕")
}

#[cfg(test)]
mod tests {
    use super::{describe_text, make_greeting};

    #[test]
    fn bytes_and_characters_can_differ() {
        assert_eq!(describe_text("café ☕"), (9, 6));
    }

    #[test]
    fn builds_an_owned_greeting() {
        assert_eq!(make_greeting("Ada"), "¡Hola, Ada! ☕");
    }
}
