use pyo3::prelude::*;
use tritium_remote;

#[pyclass]
pub struct TritiumConnection {
    pub inner: tritium_remote::Connection

    // #[pyo3(get, set)]
    // pub foo: i32,
}

// async fn mock_async_connect(url: &str) -> TritiumConnection {
//     println!("pretending to connect to {}", url);
//     async_std::task::sleep(Duration::from_secs(1)).await;

//     TritiumConnection { foo: 123 }
// }

// #[pyfunction]
// fn connect(py: Python, url: String) -> PyResult<&PyAny> {
//     pyo3_asyncio::tokio::future_into_py(py, async move {
//         let c = mock_async_connect(&url).await;

//         Ok(c)
//     })
// }

#[pyfunction]
fn connect(py: Python, url: String) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let connection = tritium_remote::connect(&url).await;
        Ok(TritiumConnection {
            inner:connection
        })
    })
}

#[pyfunction]
fn do_something(py: Python, connection: ???) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        tritium_remote::do_something(connection.inner).await;
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
    m.add_function(wrap_pyfunction!(connect, m)?)?;
    m.add_function(wrap_pyfunction!(do_something, m)?)?;
    Ok(())
}
