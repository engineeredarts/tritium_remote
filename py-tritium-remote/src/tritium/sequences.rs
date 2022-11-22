use pyo3::prelude::*;

use crate::Tritium;
use crate::TritiumError;

#[pyclass]
pub struct PlayingSequence {
    #[pyo3(get, set)]
    id: String,
}

#[pymethods]
impl Tritium {
    pub fn play_sequence<'p>(
        &mut self,
        py: Python<'p>,
        project_path: String,
    ) -> PyResult<&'p PyAny> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut tritium = inner.lock().await;
            let playing_sequence = tritium
                .play_sequence(&project_path)
                .await
                .map_err(|err| TritiumError::new_err(err.to_string()))?;

            Ok(PlayingSequence {
                id: playing_sequence.id,
            })
        })
    }
}
