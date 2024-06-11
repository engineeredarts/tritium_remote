use simple_logger::SimpleLogger;
use std::env;

#[tokio::main]
async fn main() {
    // logging controlled by the RUST_LOG environment variable
    SimpleLogger::new().env().init().unwrap();

    let host = env::var("TRITIUM_HOST").unwrap_or("localhost".to_string());

    let auth_token =
        env::var("TRITIUM_AUTH_TOKEN").expect("TRITIUM_AUTH_TOKEN environment variable not set");

    let url = format!("ws://{host}:1234");

    let mut tritium = tritium_remote::connect(
        &url,
        &auth_token,
        Some("tritium-remote example - system info".to_string()),
    )
    .await
    .expect("failed to connect");

    let system_info = tritium
        .query_basic_system_info()
        .await
        .expect("query failed");

    log::info!("System info:");
    log::info!("  serial: {}", system_info.serial);
    log::info!(
        "  name: {}",
        system_info.name.unwrap_or("(no name)".to_string())
    );
    log::info!("  version: {}", system_info.version);
}
