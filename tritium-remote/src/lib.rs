mod client;
mod graphql;
mod protocol;
mod tokio_spawner;

pub mod tritium;
use tritium::Tritium;

pub mod error;
use error::TritiumError;

use client::GatewayGraphQLClientBuilder;

pub async fn connect(url: &str, auth_token: &str) -> Result<Tritium, TritiumError> {
    let client = GatewayGraphQLClientBuilder::new()
        .build(url, auth_token)
        .await?;
    Ok(Tritium::new(client))
}
