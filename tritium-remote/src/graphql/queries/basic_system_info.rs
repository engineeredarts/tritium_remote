use graphql_client::{GraphQLQuery, QueryBody};
use serde::{Deserialize, Serialize};

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

//////////////////////////////////////////////////////////////////////////////

const OPERATION_NAME: &str = "BasicSystemInfo";

const QUERY: &str = "
query BasicSystemInfo {
    system {
        serial
        version
        name
    }
}";

pub struct BasicSystemInfo;

impl GraphQLQuery for BasicSystemInfo {
    type Variables = Variables;
    type ResponseData = ResponseData;

    fn build_query(variables: Self::Variables) -> QueryBody<Self::Variables> {
        QueryBody {
            variables,
            query: QUERY,
            operation_name: OPERATION_NAME,
        }
    }
}
