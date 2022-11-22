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

pub async fn connect(url: &str, auth_token: &str) -> Result<Tritium, TritiumError> {
    let client = GatewayGraphQLClientBuilder::new()
        .build(url, auth_token)
        .await?;
    Ok(Tritium::new(client))
}
