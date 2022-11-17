use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let mut c = tritium_remote::connect("ws://localhost:1234").await;

    let system_info = tritium_remote::query_basic_system_info(&mut c)
        .await
        .unwrap();
    println!("System info: {}", system_info.serial);

    sleep(Duration::from_millis(100)).await;
}
