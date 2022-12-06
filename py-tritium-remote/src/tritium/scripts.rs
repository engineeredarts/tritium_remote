use pyo3::prelude::*;

use crate::Tritium;
use crate::TritiumError;

#[pyclass]
pub struct Script {
    // #[pyo3(get, set)]
// path: Option<String>,
}

#[pymethods]
impl Tritium {
    pub fn start_script<'p>(&mut self, py: Python<'p>, script_path: String) -> PyResult<&'p PyAny> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut tritium = inner.lock().await;
            let _script = tritium
                .start_script(&script_path)
                .await
                .map_err(|err| TritiumError::new_err(err.to_string()))?;

            Ok(Script { /*path: script.path*/ })
        })
    }

    pub fn stop_script<'p>(&mut self, py: Python<'p>, script_path: String) -> PyResult<&'p PyAny> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut tritium = inner.lock().await;
            let _script = tritium
                .stop_script(&script_path)
                .await
                .map_err(|err| TritiumError::new_err(err.to_string()))?;

            Ok(Script { /*path: script.path*/ })
        })
    }
}
