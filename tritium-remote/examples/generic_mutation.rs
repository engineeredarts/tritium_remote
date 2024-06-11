use simple_logger::SimpleLogger;
use std::env;

use serde_json::json;

#[tokio::main]
async fn main() {
    // logging controlled by the RUST_LOG environment variable
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
        mutation trigger($input:ScriptTriggerInput!) {
            manuallyTriggerScript(input: $input) {
                script {
                    status
                }
            }
        }    
    ";

    log::info!("Document: {document}");

    let variables = json!( {
        "input": {
            "action": "START",
            "path": "start_stop.py"
        }
    });
    log::info!("Variables: {variables:?}");

    let result = tritium
        .query(&document, Some(variables))
        .await
        .expect("query failed");

    log::info!("Result: {result:?}");
}
