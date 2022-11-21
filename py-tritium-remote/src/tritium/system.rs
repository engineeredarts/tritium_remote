use pyo3::prelude::*;

use crate::Tritium;
use crate::TritiumError;

#[pyclass]
pub struct TritiumSystemInfo {
    #[pyo3(get, set)]
    serial: String,

    #[pyo3(get, set)]
    name: Option<String>,

    #[pyo3(get, set)]
    version: String,
}

#[pymethods]
impl Tritium {
    pub fn query_system_info<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut tritium = inner.lock().await;
            let system_info = tritium
                .query_basic_system_info()
                .await
                .map_err(|err| TritiumError::new_err(err.to_string()))?;

            Ok(TritiumSystemInfo {
                serial: system_info.serial,
                name: system_info.name,
                version: system_info.version,
            })
        })
    }
}
