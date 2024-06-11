use simple_logger::SimpleLogger;
use std::env;

#[tokio::main]
async fn main() {
    // logging controlled by the RUST_LOG environment variable
    SimpleLogger::new().env().init().unwrap();

    let auth_token =
        env::var("TRITIUM_AUTH_TOKEN").expect("TRITIUM_AUTH_TOKEN environment variable not set");

    let mut tritium = tritium_remote::connect(
        "ws://localhost:1234",
        &auth_token,
        Some("tritium-remote example - generic subscription".to_string()),
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

    log::info!("Document: {document}");

    let mut sub = tritium
        .subscription(&document, None)
        .await
        .expect("subscription failed");

    log::info!("Subscription: {sub:?}");

    loop {
        match sub.results.recv().await {
            Some(r) => {
                let data = r.data;
                log::info!("Subscription data: {data:#?}");
            }
            None => {
                break;
            }
        }
    }
}
