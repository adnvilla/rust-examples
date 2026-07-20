#[must_use]
pub fn database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://rust:rust@127.0.0.1:5432/rust_examples".to_owned())
}

#[cfg(test)]
mod tests {
    use super::database_url;

    #[test]
    fn default_url_targets_the_ephemeral_service() {
        assert!(database_url().starts_with("postgres://"));
    }
}
