#[must_use]
pub fn redis_url() -> String {
    std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_owned())
}

#[cfg(test)]
mod tests {
    use super::redis_url;

    #[test]
    fn default_url_targets_the_ephemeral_cache() {
        assert!(redis_url().starts_with("redis://"));
    }
}
