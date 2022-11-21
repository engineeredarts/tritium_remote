use thiserror::Error;
use tokio::sync::mpsc;

/// TritiumError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum TritiumError {
    #[error("Authentication error: {0:?}")]
    AuthenticationError(String),

    #[error("Communication error: {0:?}")]
    CommunicationError(String),

    #[error("Error: {0:?}")]
    GenericError(String),
}

impl From<tungstenite::error::Error> for TritiumError {
    fn from(err: tungstenite::error::Error) -> Self {
        TritiumError::CommunicationError(err.to_string())
    }
}

impl<T> From<mpsc::error::SendError<T>> for TritiumError {
    fn from(err: mpsc::error::SendError<T>) -> Self {
        TritiumError::CommunicationError(err.to_string())
    }
}
