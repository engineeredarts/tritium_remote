use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tritium_remote;

// mod error;
// use error::PyTritiumError;

#[pyclass]
pub struct Tritium {
    inner: Arc<Mutex<tritium_remote::Tritium>>,
}

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
                .map_err(|err| PyException::new_err(err.to_string()))?;

            Ok(TritiumSystemInfo {
                serial: system_info.serial,
                name: system_info.name,
                version: system_info.version,
            })
        })
    }
}

#[pyfunction]
fn connect(py: Python, url: String, auth_token: String) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let tritium = tritium_remote::connect(&url, &auth_token)
            .await
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(Tritium {
            inner: Arc::new(Mutex::new(tritium)),
        })
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_tritium_remote(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(connect, m)?)?;
    Ok(())
}
