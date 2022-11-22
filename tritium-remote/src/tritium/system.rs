use crate::error::TritiumError;
use crate::graphql::queries::basic_system_info::{basic_system_info, BasicSystemInfo};
use crate::graphql::QueryOperation;
use crate::tritium::Tritium;

impl Tritium {
    pub async fn query_basic_system_info(&mut self) -> Result<SystemInfo, TritiumError> {
        let operation = QueryOperation::<BasicSystemInfo>::new(basic_system_info::Variables {});
        let query = self.client.graphql_query(operation).await?;
        let response = query.result.await?;

        // TODO - generic way to extract data or return errors
        if let Some(errors) = response.errors {
            return Err(TritiumError::from(errors));
        }

        match response.data {
            Some(data) => Ok(SystemInfo::from(data.system)),
            _ => Err(TritiumError::GenericError(
                "GraphQL response contained no data".to_string(),
            )),
        }
    }
}

pub struct SystemInfo {
    pub serial: String,
    pub version: String,
    pub name: Option<String>,
}

impl From<basic_system_info::BasicSystemInfoSystem> for SystemInfo {
    fn from(info: basic_system_info::BasicSystemInfoSystem) -> SystemInfo {
        SystemInfo {
            serial: info.serial,
            version: info.version,
            name: info.name,
        }
    }
}
