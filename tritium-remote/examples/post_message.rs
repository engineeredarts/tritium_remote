// use simple_logger::SimpleLogger;
use std::env;
use std::time::SystemTime;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // output *all* log messages, including from underlying transport
    // SimpleLogger::new().env().init().unwrap();

    let auth_token =
        env::var("TRITIUM_AUTH_TOKEN").expect("TRITIUM_AUTH_TOKEN environment variable not set");

    let mut tritium = tritium_remote::connect(
        "ws://localhost:1234",
        &auth_token,
        Some("tritium-remote example - post message".to_string()),
    )
    .await
    .expect("failed to connect");

    loop {
        let t = seconds_since_unix_epoch();
        let message = format!("The remote time is now {t}s since the start of 1970");
        println!("posting to channel \"talking_clock\": {message}");
        tritium
            .post_message("talking_clock", message)
            .await
            .expect("post failed");

        sleep(Duration::from_secs(1)).await;
    }
}

fn seconds_since_unix_epoch() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
