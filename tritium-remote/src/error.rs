use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum CommunicationErrorInner {
    Unknown,
    Generic(String),
} 

/// TritiumError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum TritiumError {
    #[error("Communication error: {0:?}")]
    CommunicationError(CommunicationErrorInner),

    #[error("Error: {0:?}")]
    GenericError(String),
}

impl TritiumError {
    pub fn comms_from_str(msg: &str) -> Self {
        TritiumError::CommunicationError(CommunicationErrorInner::Generic(msg.to_string()))
    }

    pub fn generic(msg: String) -> Self {
        TritiumError::GenericError(msg)
    }

    pub fn generic_from_str(msg: &str) -> Self {
        TritiumError::GenericError(msg.to_string())
    }
}

impl<T> From<mpsc::error::SendError<T>> for TritiumError {
    fn from(_err: mpsc::error::SendError<T>) -> Self {
        TritiumError::CommunicationError(CommunicationErrorInner::Unknown)
    }
}

impl From<CommunicationErrorInner> for TritiumError {
    fn from(inner: CommunicationErrorInner) -> Self {
        TritiumError::CommunicationError(inner)
    }
}

// pub type Result<T> = std::result::Result<T, TritiumError>;
