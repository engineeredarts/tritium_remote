use tritium_remote::connect;

#[tokio::test]
async fn it_connects() {
    let c = connect("localhost:1234").await;
    assert!(c.open)
}
