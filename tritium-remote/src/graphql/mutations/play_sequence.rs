#![allow(clippy::all, warnings)]
pub struct PlaySequence;
pub mod play_sequence {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PlaySequence";
    pub const QUERY : & str = "mutation PlaySequence($input: PlaySequenceInput!) {\n    playSequence(input: $input) {\n        id\n    }\n}\n" ;
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
    pub struct PlaySequenceInput {
        #[serde(rename = "projectPath")]
        pub project_path: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub input: PlaySequenceInput,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "playSequence")]
        pub play_sequence: PlaySequencePlaySequence,
    }
    #[derive(Deserialize)]
    pub struct PlaySequencePlaySequence {
        pub id: String,
    }
}
impl graphql_client::GraphQLQuery for PlaySequence {
    type Variables = play_sequence::Variables;
    type ResponseData = play_sequence::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: play_sequence::QUERY,
            operation_name: play_sequence::OPERATION_NAME,
        }
    }
}
