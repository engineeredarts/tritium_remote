use serde_json::json;

use crate::error::TritiumError;
use crate::graphql::{GenericResponse, GenericSubscriptionResponse};
use crate::tritium::Tritium;

pub type Variables = serde_json::Value;

impl Tritium {
    pub async fn query(
        &mut self,
        document: &str,
        variables: Option<Variables>,
    ) -> Result<GenericResponse, TritiumError> {
        let variables = variables.unwrap_or_else(|| json!(null));

        let query = self
            .client
            .generic_graphql_query(document, variables)
            .await?;
        let response = query.result.await?;

        Ok(response)
    }

    pub async fn subscription(
        &mut self,
        _document: &str,
        _variables: Option<Variables>,
    ) -> Result<GenericSubscriptionResponse, TritiumError> {
        todo!()
    }
}
