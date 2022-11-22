use graphql_client::{GraphQLQuery, QueryBody, Response};
use serde_json::json;

pub mod mutations;
pub mod queries;

pub type GenericResponse = Response<serde_json::Value>;

/// An abstraction over GraphQL operations.
pub trait GraphQLOperation {
    /// The actual response & error type of this operation.
    type Response;

    /// The error that will be returned from failed attempts to decode a `Response`.
    type Error: std::error::Error;

    /// Decodes a `GenericResponse` into the actual response that will be returned
    /// to users for this operation.
    fn decode(&self, data: GenericResponse) -> Result<Self::Response, Self::Error>;

    fn get_document(&self) -> &str;

    fn get_variables(&self) -> serde_json::Value;
}

pub struct QueryOperation<Q: GraphQLQuery> {
    inner: QueryBody<Q::Variables>,
}

impl<Q: GraphQLQuery> QueryOperation<Q> {
    pub fn new(variables: Q::Variables) -> Self {
        Self {
            inner: Q::build_query(variables),
        }
    }

    fn decode_response(
        &self,
        response: Response<serde_json::Value>,
    ) -> Result<Response<Q::ResponseData>, serde_json::Error> {
        if let Some(data) = response.data {
            Ok(::graphql_client::Response {
                data: Some(serde_json::from_value(data)?),
                errors: response.errors,
                extensions: response.extensions,
            })
        } else {
            Ok(::graphql_client::Response {
                data: None,
                errors: response.errors,
                extensions: response.extensions,
            })
        }
    }
}

impl<Q: GraphQLQuery> GraphQLOperation for QueryOperation<Q> {
    type Response = Response<Q::ResponseData>;

    type Error = serde_json::Error;

    fn decode(&self, response: GenericResponse) -> Result<Self::Response, Self::Error> {
        self.decode_response(response)
    }

    fn get_document(&self) -> &str {
        return self.inner.query;
    }

    fn get_variables(&self) -> serde_json::Value {
        let v = serde_json::to_value(&self.inner.variables).unwrap();
        match v {
            serde_json::Value::Object(_) => v,
            _ => json!({}),
        }
    }
}
