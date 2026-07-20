#[must_use]
pub fn most_detailed<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.chars().count() >= right.chars().count() {
        left
    } else {
        right
    }
}

#[derive(Debug)]
pub struct Review<'a> {
    pub author: &'a str,
    pub text: &'a str,
}

impl Review<'_> {
    #[must_use]
    pub fn summary(&self) -> String {
        format!("{} opina: {}", self.author, self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::most_detailed;

    #[test]
    fn result_borrows_from_an_input() {
        let short = String::from("Bien");
        let detailed = String::from("Volvería por el postre");
        assert_eq!(most_detailed(&short, &detailed), detailed);
    }
}
