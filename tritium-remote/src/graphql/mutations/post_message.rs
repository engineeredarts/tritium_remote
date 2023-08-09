#![allow(clippy::all, warnings)]
pub struct PostMessage;
pub mod post_message {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "postMessage";
    pub const QUERY: &str =
        "mutation PostMessage($input: PostMessageInput!) {\n   postMessage(input: $input) }\n";
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct PostMessageInput {
        pub channel: String,
        pub message: serde_json::Value,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub input: PostMessageInput,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {}
}
impl graphql_client::GraphQLQuery for PostMessage {
    type Variables = post_message::Variables;
    type ResponseData = post_message::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: post_message::QUERY,
            operation_name: post_message::OPERATION_NAME,
        }
    }
}
