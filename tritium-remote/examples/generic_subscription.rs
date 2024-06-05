use simple_logger::SimpleLogger;
use std::env;

#[tokio::main]
async fn main() {
    // output *all* log messages, including from underlying transport
    SimpleLogger::new().env().init().unwrap();

    let auth_token =
        env::var("TRITIUM_AUTH_TOKEN").expect("TRITIUM_AUTH_TOKEN environment variable not set");

    let mut tritium = tritium_remote::connect(
        "ws://localhost:1234",
        &auth_token,
        Some("tritium-remote example - generic mutation".to_string()),
    )
    .await
    .expect("failed to connect");

    let document: &str = "
        subscription {
            worldEvents {
                timestamp
                eventData
            }
        }
    ";

    println!("Document: {document}");

    let result = tritium
        .subscription(&document, None)
        .await
        .expect("subscription failed");

    println!("Result: {result:?}");
}
