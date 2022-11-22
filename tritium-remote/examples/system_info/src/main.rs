use simple_logger::SimpleLogger;
use std::env;

#[tokio::main]
async fn main() {
    // output *all* log messages, including from underlying transport
    SimpleLogger::new().env().init().unwrap();

    let auth_token =
        env::var("TRITIUM_AUTH_TOKEN").expect("TRITIUM_AUTH_TOKEN environment variable not set");

    let mut tritium = tritium_remote::connect("ws://localhost:1234", &auth_token)
        .await
        .expect("failed to connect");

    let system_info = tritium
        .query_basic_system_info()
        .await
        .expect("query failed");

    println!("System info:");
    println!("  serial: {}", system_info.serial);
    println!(
        "  name: {}",
        system_info.name.unwrap_or("(no name)".to_string())
    );
    println!("  version: {}", system_info.version);
}
