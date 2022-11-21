use crate::error::TritiumError;
use crate::graphql::basic_system_info::{
    basic_system_info, basic_system_info::BasicSystemInfoSystem, BasicSystemInfo,
};
use crate::graphql::QueryOperation;
use crate::tritium::Tritium;

impl Tritium {
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
}
