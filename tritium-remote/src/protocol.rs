#[derive(serde::Serialize, Debug)]
#[serde(tag = "type")]
pub enum MessageToGateway<'a, Operation> {
    #[serde(rename = "graphql")]
    GraphQL {
        request_id: u64,
        data: &'a Operation,
    },
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessageFromGateway<Response> {
    #[serde(rename = "graphql_response")]
    GraphQLResponse { request_id: u64, data: Response },
}
