#[derive(serde::Serialize, Debug)]
#[serde(tag = "type")]
pub enum MessageToGateway<'a> {
    #[serde(rename = "graphql")]
    GraphQL {
        auth_token: &'a str,
        request_id: u64,
        document: &'a str,
        variable_values: serde_json::Value,
    },
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessageFromGateway<Response> {
    #[serde(rename = "graphql_response")]
    GraphQLResponse {
        request_id: u64,
        data: Option<Response>,
        error: Option<String>,
    },
}
