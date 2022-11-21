use crate::error::TritiumError;
use crate::graphql::QueryOperation;

use crate::client::GatewayGraphQLClient;

pub struct Tritium {
    client: GatewayGraphQLClient,
}

use crate::graphql::basic_system_info::{
    basic_system_info, basic_system_info::BasicSystemInfoSystem, BasicSystemInfo,
};

impl Tritium {
    pub fn new(client: GatewayGraphQLClient) -> Self {
        Self { client }
    }

    pub async fn query_basic_system_info(&mut self) -> Result<BasicSystemInfoSystem, TritiumError> {
        let operation = QueryOperation::<BasicSystemInfo>::new(basic_system_info::Variables {});
        let query = self.client.graphql_query(operation).await?;
        let response = query.result.await?;

        match response.data {
            Some(data) => Ok(data.system),
            _ => Err(TritiumError::GenericError(
                "GraphQL response contained no data".to_string(),
            )),
        }
    }

    // pub async fn start_script(&mut self)
}
