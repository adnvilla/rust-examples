use async_tasks::fetch_all;

#[tokio::main]
async fn main() {
    for message in fetch_all(&[(1, 80), (2, 20), (3, 40)]).await {
        println!("{message}");
    }
}
