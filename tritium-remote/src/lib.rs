mod client;
mod protocol;
mod tokio_spawner;

mod auth;
mod graphql;

mod error;
use error::TritiumError;

use client::{GatewayGraphQLClient, GatewayGraphQLClientBuilder};

use graphql::basic_system_info::basic_system_info::BasicSystemInfoSystem;
use graphql::basic_system_info::BasicSystemInfo;
use graphql::QueryOperation;

pub struct Connection {
    client: GatewayGraphQLClient,
}

pub async fn connect(url: &str) -> Result<Connection, TritiumError> {
    let client = GatewayGraphQLClientBuilder::new().build(url).await?;
    Ok(Connection { client })
}

impl Connection {
    pub async fn query_basic_system_info(&mut self) -> Result<BasicSystemInfoSystem, TritiumError> {
        let operation = QueryOperation::<BasicSystemInfo>::new(
            graphql::basic_system_info::basic_system_info::Variables {},
        );
        let query = self.client.graphql_query(operation).await?;
        let response = query.result.await?;

        match response.data {
            Some(data) => Ok(data.system),
            _ => Err(TritiumError::GenericError(
                "GraphQL response contained no data".to_string(),
            )),
        }
    }
}
