use tritium_remote::connect;

#[tokio::test]
async fn it_connects() {
    let c = connect("ws://localhost:1234").await;
    assert!(c.open)
}
