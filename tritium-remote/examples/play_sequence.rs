use std::env;

const PROJECT_PATH: &str = "test_sequence";

#[tokio::main]
async fn main() {
    let auth_token =
        env::var("TRITIUM_AUTH_TOKEN").expect("TRITIUM_AUTH_TOKEN environment variable not set");

    let mut tritium = tritium_remote::connect(
        "ws://localhost:1234",
        &auth_token,
        Some("tritium-remote example - play sequence".to_string()),
    )
    .await
    .expect("failed to connect");

    println!("Playing sequence {}", PROJECT_PATH);
    let playing_sequence = tritium
        .play_sequence(PROJECT_PATH)
        .await
        .expect("play sequence mutation failed");

    println!("OK, play_sequence returned: {:?}", playing_sequence);
}
