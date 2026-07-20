use std::time::Duration;

use tokio::{task::JoinSet, time};

pub async fn fetch_table(table: u16, delay_ms: u64) -> String {
    time::sleep(Duration::from_millis(delay_ms)).await;
    format!("mesa {table} lista")
}

pub async fn fetch_all(tables: &[(u16, u64)]) -> Vec<String> {
    let mut tasks = JoinSet::new();
    for &(table, delay) in tables {
        tasks.spawn(fetch_table(table, delay));
    }
    let mut messages = tasks.join_all().await;
    messages.sort();
    messages
}

#[cfg(test)]
mod tests {
    use super::fetch_all;

    #[tokio::test]
    async fn tasks_complete_concurrently() {
        assert_eq!(
            fetch_all(&[(2, 1), (1, 2)]).await,
            ["mesa 1 lista", "mesa 2 lista"]
        );
    }
}
