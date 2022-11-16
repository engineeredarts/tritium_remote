
#[derive(serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessageToGateway<Response> {
    #[serde(rename = "graphql")]
    GraphQL { request_id: u64, data: Response },
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessageFromGateway<Response> {
    #[serde(rename = "graphql_response")]
    GraphQLResponse { request_id: u64, data: Response },
}