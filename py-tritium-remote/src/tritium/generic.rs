use serde::Serialize;
use pyo3::prelude::*;

use crate::Tritium;
use crate::TritiumError;
use crate::graphql::GenericResponse;

#[derive(Serialize)]
struct Response {
    data: Option<serde_json::Value>
}

#[pymethods]
impl Tritium {
    pub fn query<'p>(&mut self, py: Python<'p>, query: String) -> PyResult<&'p PyAny> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut tritium = inner.lock().await;
            let response = tritium
                .query(&query, None)
                .await
                .map_err(|err| TritiumError::new_err(err.to_string()))?;

            let GenericResponse { data, .. } = response;

            let r = Response {
                data
            }

            let response_json = serde_json::to_str(r).map_err(|err| TritiumError::new_err(err.to_string()))?
            Ok(respons_json)
        })
    }
}
