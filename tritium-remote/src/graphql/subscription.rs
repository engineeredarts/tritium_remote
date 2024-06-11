use tokio::sync::mpsc;

use super::operation::GenericResponse;

#[derive(Debug)]
pub struct GenericSubscription {
    pub results: mpsc::UnboundedReceiver<GenericResponse>,
}
