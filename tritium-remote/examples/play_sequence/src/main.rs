use std::env;

#[tokio::main]
async fn main() {
    let auth_token =
        env::var("TRITIUM_AUTH_TOKEN").expect("TRITIUM_AUTH_TOKEN environment variable not set");

    let mut tritium = tritium_remote::connect("ws://localhost:1234", &auth_token)
        .await
        .expect("failed to connect");

    tritium
        .play_sequence("test_sequence")
        .await
        .expect("play sequence mutation failed");
}