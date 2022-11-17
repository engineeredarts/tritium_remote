use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tritium_remote;

#[pyclass]
pub struct Tritium {
    inner: Arc<Mutex<tritium_remote::Connection>>,
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
            let mut c = inner.lock().await;
            let system_info = tritium_remote::query_basic_system_info(&mut c)
                .await
                .unwrap();

            Ok(TritiumSystemInfo {
                serial: system_info.serial,
                name: system_info.name,
                version: system_info.version,
            })
        })
    }
}

#[pyfunction]
fn connect(py: Python, url: String) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let connection = tritium_remote::connect(&url).await;
        Ok(Tritium {
            inner: Arc::new(Mutex::new(connection)),
        })
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_tritium_remote(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(connect, m)?)?;
    Ok(())
}
