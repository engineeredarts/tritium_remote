use tritium_remote::connect;

#[test]
fn it_connects() {
    let c = connect("localhost:1234");
    assert!(c.open)
}
