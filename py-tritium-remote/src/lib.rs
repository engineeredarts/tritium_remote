use pyo3::prelude::*;
use std::time::Duration;

/// Sleep for 1 second
async fn rust_sleep() {
    async_std::task::sleep(Duration::from_secs(1)).await;
}

#[pyfunction]
fn call_rust_sleep(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::async_std::future_into_py(py, async move {
        rust_sleep().await;
        Ok(())
    })
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_tritium_remote(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(call_rust_sleep, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
