use std::env;

const SCRIPT_PATH: &str = "start_stop.py";

#[tokio::main]
async fn main() {
    let host = env::var("TRITIUM_HOST").unwrap_or("localhost".to_string());

    let auth_token =
        env::var("TRITIUM_AUTH_TOKEN").expect("TRITIUM_AUTH_TOKEN environment variable not set");

    let url = format!("ws://{host}:1234");

    let mut tritium = tritium_remote::connect(
        &url,
        &auth_token,
        Some("tritium-remote example - start script".to_string()),
    )
    .await
    .expect("failed to connect");

    println!("Starting script {}", SCRIPT_PATH);
    let script = tritium
        .start_script(SCRIPT_PATH)
        .await
        .expect("trigger script mutation failed");

    println!("OK, start_script returned: {:?}", script);
}
