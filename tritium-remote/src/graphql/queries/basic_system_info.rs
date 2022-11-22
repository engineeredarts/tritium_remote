#![allow(clippy::all, warnings)]
pub struct BasicSystemInfo;
pub mod basic_system_info {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "BasicSystemInfo";
    pub const QUERY : & str = "query BasicSystemInfo {\n    system {\n        serial\n        version\n        name\n    }\n}\n" ;
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
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub system: BasicSystemInfoSystem,
    }
    #[derive(Deserialize)]
    pub struct BasicSystemInfoSystem {
        pub serial: String,
        pub version: String,
        pub name: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for BasicSystemInfo {
    type Variables = basic_system_info::Variables;
    type ResponseData = basic_system_info::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: basic_system_info::QUERY,
            operation_name: basic_system_info::OPERATION_NAME,
        }
    }
}
