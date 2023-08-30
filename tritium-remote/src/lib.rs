mod client;
mod graphql;
mod protocol;
mod tokio_spawner;

mod tritium;

pub use tritium::scripts::{Script, ScriptStatus};
pub use tritium::sequences::PlayingSequence;
pub use tritium::system::TritiumSystemInfo;
pub use tritium::Tritium;

mod error;
pub use error::TritiumError;

use client::GatewayGraphQLClientBuilder;

/// Connects to a Tritium system via an unsecured WebSocket.  
///
/// Arguments:
/// * `url`: The system WebSocket address in the form _ws://localhost:1234_
/// * `auth_token`: JWT access token string granting access
pub async fn connect(
    url: &str,
    auth_token: &str,
    description: Option<String>,
) -> Result<Tritium, TritiumError> {
    let client = GatewayGraphQLClientBuilder::new()
        .build(url, auth_token, description)
        .await?;
    Ok(Tritium::new(client))
}
