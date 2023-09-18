use pyo3::prelude::*;
use serde::Serialize;

use crate::Tritium;
use crate::TritiumError;

#[derive(Serialize)]
struct Response {
    data: Option<serde_json::Value>,
    errors: Option<Vec<tritium_remote::graphql_client::Error>>,
}

#[pymethods]
impl Tritium {
    /// Executes a GraphQL document supplied by the client, returning any
    /// response as a JSON encoded object.
    pub fn query<'p>(
        &mut self,
        py: Python<'p>,
        document: String,
        variables_json: Option<String>,
    ) -> PyResult<&'p PyAny> {
        let inner = self.inner.clone();
        let variables: Option<serde_json::Value> =
            variables_json.map(|j| serde_json::from_str(&j).unwrap());
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut tritium = inner.lock().await;
            let r = tritium
                .query(&document, variables)
                .await
                .map_err(|err| TritiumError::new_err(err.to_string()))?;

            let response = Response {
                data: r.data,
                errors: r.errors,
            };

            let response_json = serde_json::to_string(&response)
                .map_err(|err| TritiumError::new_err(err.to_string()))?;

            Ok(response_json)
        })
    }
}
