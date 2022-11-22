use thiserror::Error;
use tokio::sync::mpsc;

/// TritiumError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum TritiumError {
    #[error("Communication error: {0}")]
    CommunicationError(String),

    #[error("{0}")]
    GenericError(String),

    #[error("GraphQL errors: {0:?}")]
    GraphQLErrors(Vec<graphql_client::Error>),
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

impl From<Vec<graphql_client::Error>> for TritiumError {
    fn from(errors: Vec<graphql_client::Error>) -> Self {
        TritiumError::GraphQLErrors(errors)
    }
}
