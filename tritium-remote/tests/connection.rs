use tokio::time::{sleep, Duration};
use tritium_remote;

#[tokio::test]
async fn it_connects() {
    tritium_remote::connect("ws://localhost:1234").await;
}

#[tokio::test]
async fn it_does_something() {
    let c = tritium_remote::connect("ws://localhost:1234").await;
    tritium_remote::do_something(&c).await.unwrap();

    sleep(Duration::from_millis(100)).await;

}
