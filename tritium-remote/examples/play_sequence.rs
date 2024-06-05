use simple_logger::SimpleLogger;
use std::env;

const PROJECT_PATH: &str = "test_sequence";

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
        Some("tritium-remote example - play sequence".to_string()),
    )
    .await
    .expect("failed to connect");

    log::info!("Playing sequence {}", PROJECT_PATH);
    let playing_sequence = tritium
        .play_sequence(PROJECT_PATH)
        .await
        .expect("play sequence mutation failed");

    log::info!("OK, play_sequence returned: {:?}", playing_sequence);
}
