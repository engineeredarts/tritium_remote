// use simple_logger::SimpleLogger;
use std::env;

#[tokio::main]
async fn main() {
    // output *all* log messages, including from underlying transport
    // SimpleLogger::new().env().init().unwrap();

    let auth_token =
        env::var("TRITIUM_AUTH_TOKEN").expect("TRITIUM_AUTH_TOKEN environment variable not set");

    let mut tritium = tritium_remote::connect(
        "ws://localhost:1234",
        &auth_token,
        Some("tritium-remote example - generic query".to_string()),
    )
    .await
    .expect("failed to connect");

    const DOCUMENT: &str = "
        query { 
            system { 
                serial  
            }
        }
    ";

    let result = tritium.query(&DOCUMENT, None).await.expect("query failed");

    println!("Result: {result}");
}
