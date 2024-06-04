use crate::error::TritiumError;
use crate::graphql::queries::basic_system_info;
use crate::graphql::queries::basic_system_info::BasicSystemInfo;
use crate::graphql::QueryOperation;
use crate::tritium::Tritium;

/// General system methods.
impl Tritium {
    /// Queries and returns basic information about the system.
    /// * Serial number / identifier
    /// * Human readable name, if any
    /// * Tritium version
    pub async fn query_basic_system_info(&mut self) -> Result<TritiumSystemInfo, TritiumError> {
        let operation = QueryOperation::<BasicSystemInfo>::new(basic_system_info::Variables {});
        let query = self.client.graphql_query(operation).await?;
        let response = query.result.await?;

        // TODO - generic way to extract data or return errors
        if let Some(errors) = response.errors {
            return Err(TritiumError::from(errors));
        }

        match response.data {
            Some(data) => Ok(TritiumSystemInfo::from(data.system)),
            _ => Err(TritiumError::GenericError(
                "GraphQL response contained no data".to_string(),
            )),
        }
    }
}

/// Tritium system information.
#[derive(Debug)]
pub struct TritiumSystemInfo {
    /// Serial number / identifier
    pub serial: String,
    /// Tritium version
    pub version: String,
    /// Human readable name, if any
    pub name: Option<String>,
}

impl From<basic_system_info::BasicSystemInfoSystem> for TritiumSystemInfo {
    fn from(info: basic_system_info::BasicSystemInfoSystem) -> TritiumSystemInfo {
        TritiumSystemInfo {
            serial: info.serial,
            version: info.version,
            name: info.name,
        }
    }
}
