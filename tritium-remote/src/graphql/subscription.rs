use futures::channel::mpsc;

use super::operation::GenericResponse;

#[derive(Debug)]
pub struct GenericSubscription {
    pub results: mpsc::Receiver<GenericResponse>,
}
