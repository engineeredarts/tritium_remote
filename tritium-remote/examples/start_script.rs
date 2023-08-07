use std::env;

const SCRIPT_PATH: &str = "start_stop.py";

#[tokio::main]
async fn main() {
    let auth_token =
        env::var("TRITIUM_AUTH_TOKEN").expect("TRITIUM_AUTH_TOKEN environment variable not set");

    let mut tritium = tritium_remote::connect("ws://localhost:1234", &auth_token)
        .await
        .expect("failed to connect");

    println!("Starting script {}", SCRIPT_PATH);
    let script = tritium
        .start_script(SCRIPT_PATH)
        .await
        .expect("trigger script mutation failed");

    println!("OK, start_script returned: {:?}", script);
}
