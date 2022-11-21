// use pyo3::exceptions::PyException;
// use pyo3::PyErr;
// use tritium_remote::error::TritiumError;

// pub struct PyTritiumError(TritiumError);

// impl From<PyTritiumError> for PyErr {
//     fn from(error: PyTritiumError) -> Self {
//         PyException::new_err(error.0.to_string())
//     }
// }

// impl From<TritiumError> for PyTritiumError {
//     fn from(other: TritiumError) -> Self {
//         Self(other)
//     }
// }
