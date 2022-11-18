#[derive(serde::Serialize, Debug)]
#[serde(tag = "type")]
pub enum MessageToGateway<'a> {
    #[serde(rename = "graphql")]
    GraphQL {
        auth_token: &'a str,
        request_id: u64,
        document: &'a str,
    },
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessageFromGateway<Response> {
    #[serde(rename = "graphql_response")]
    GraphQLResponse { request_id: u64, data: Response },
}
