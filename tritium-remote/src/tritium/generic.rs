use serde_json::json;

use crate::error::TritiumError;
use crate::tritium::Tritium;

pub type Variables = serde_json::Value;
pub type ResponseData = serde_json::Value;

impl Tritium {
    pub async fn query(
        &mut self,
        document: &str,
        variables: Option<Variables>,
    ) -> Result<ResponseData, TritiumError> {
        let variables = variables.unwrap_or_else(|| json!(null));

        let query = self
            .client
            .generic_graphql_query(document, variables)
            .await?;
        let response = query.result.await?;

        match response.data {
            Some(data) => Ok(data),
            None => Err(TritiumError::GenericError("no response data".to_string())),
        }
    }
}
