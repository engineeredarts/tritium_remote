use std::sync::Arc;
use tokio::sync::Mutex;

use pyo3::create_exception;
use pyo3::prelude::*;

mod tritium;

create_exception!(tritium, TritiumError, pyo3::exceptions::PyException);

/// A remote Tritium system
#[pyclass]
pub struct Tritium {
    inner: Arc<Mutex<::tritium_remote::Tritium>>,
}

/// Connects to the given remote Tritium system using an unencrypted WebSocket.
///
/// Arguments:
///   - url: The URL to connect to in the form ws://xxx.xxx.xxx.xxx:1234
///   - auth_token: JWT authentication token
/// Returns: Tritium
/// Raises: TritiumError if unable to connect
#[pyfunction]
fn connect(py: Python, url: String, auth_token: String) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let tritium = ::tritium_remote::connect(&url, &auth_token)
            .await
            .map_err(|err| TritiumError::new_err(err.to_string()))?;
        Ok(Tritium {
            inner: Arc::new(Mutex::new(tritium)),
        })
    })
}

/// Functions and classes for interacting with a remote Tritium system.
#[pymodule]
fn tritium_remote(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(connect, m)?)?;
    m.add("Tritium", py.get_type::<Tritium>())?;
    m.add("TritiumError", py.get_type::<TritiumError>())?;
    Ok(())
}
