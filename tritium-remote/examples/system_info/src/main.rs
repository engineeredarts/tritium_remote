#[tokio::main]
async fn main() {
    let mut tritium = tritium_remote::connect("ws://localhost:1234").await;

    let system_info = tritium_remote::query_basic_system_info(&mut tritium)
        .await
        .unwrap();

    println!("System info:");
    println!("  serial: {}", system_info.serial);
    println!(
        "  name: {}",
        system_info.name.unwrap_or("(no name)".to_string())
    );
    println!("  version: {}", system_info.version);
}
