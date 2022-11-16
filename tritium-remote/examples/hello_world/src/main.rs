use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let mut c = tritium_remote::connect("ws://localhost:1234").await;
    
    tritium_remote::hello_world(&mut c).await.unwrap();

    sleep(Duration::from_millis(100)).await;
}
