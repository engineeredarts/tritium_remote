use futures::{
    channel::{mpsc, oneshot},
    future::RemoteHandle,
    sink::Sink,
    stream::Stream,
};
use std::{collections::HashMap, sync::Arc};
use tokio;

use super::websockets::WebsocketMessage;

pub struct GatewayGraphQLClientBuilder {}

impl GatewayGraphQLClientBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn build(
        self,
        mut websocket_stream: impl Stream<Item = Result<tungstenite::Message, tungstenite::Error>>
            + Unpin
            + Send
            + 'static,
        mut websocket_sink: impl Sink<tungstenite::Message, Error = tungstenite::Error>
            + Unpin
            + Send
            + 'static,
    ) -> GatewayGraphQLClient {
        let (mut sender_sink, _sender_stream) = mpsc::channel(1);

        // start receiving & processing messages
        tokio::spawn(receiver_loop(websocket_stream, sender_sink));

        GatewayGraphQLClient {}
    }
}

pub struct GatewayGraphQLClient {}

// type OperationMap<GenericResponse> = Arc<Mutex<HashMap<Uuid, OperationSender<GenericResponse>>>>;

async fn receiver_loop<S, WsMessage, GraphqlClient>(
    mut receiver: S,
    mut sender: mpsc::Sender<WsMessage>,
    operations: OperationMap<GraphqlClient::Response>,
    shutdown: oneshot::Sender<()>,
) -> Result<(), Error>
where
    S: Stream<Item = Result<WsMessage, WsMessage::Error>> + Unpin,
    WsMessage: WebsocketMessage,
    GraphqlClient: crate::graphql::GraphqlClient,
{
    while let Some(msg) = receiver.next().await {
        trace!("Received message: {:?}", msg);
        if let Err(err) =
            handle_message::<WsMessage, GraphqlClient>(msg, &mut sender, &operations).await
        {
            trace!("message handler error, shutting down: {err:?}");
            #[cfg(feature = "no-logging")]
            let _ = err;
            break;
        }
    }

    shutdown
        .send(())
        .map_err(|_| Error::SenderShutdown("Couldn't shutdown sender".to_owned()))
}

#[derive(thiserror::Error, Debug)]
/// Error type
pub enum Error {
    /// Unknown error
    #[error("unknown: {0}")]
    Unknown(String),
    /// Custom error
    #[error("{0}: {1}")]
    Custom(String, String),
    /// Unexpected close frame
    #[error("got close frame, reason: {0}")]
    Close(String),
    /// Decoding / parsing error
    #[error("message decode error, reason: {0}")]
    Decode(String),
    /// Sending error
    #[error("message sending error, reason: {0}")]
    Send(String),
    /// Futures spawn error
    #[error("futures spawn error, reason: {0}")]
    SpawnHandle(String),
    /// Sender shutdown error
    #[error("sender shutdown error, reason: {0}")]
    SenderShutdown(String),
}

async fn sender_loop<M, S, E, GenericResponse>(
    message_stream: mpsc::Receiver<M>,
    mut ws_sender: S,
    operations: OperationMap<GenericResponse>,
    shutdown: oneshot::Receiver<()>,
) -> Result<(), Error>
where
    M: WebsocketMessage,
    S: Sink<M, Error = E> + Unpin,
    E: std::error::Error,
{
    use futures::{future::FutureExt, select};

    let mut message_stream = message_stream.fuse();
    let mut shutdown = shutdown.fuse();

    loop {
        select! {
            msg = message_stream.next() => {
                if let Some(msg) = msg {
                    trace!("Sending message: {:?}", msg);
                    ws_sender
                        .send(msg)
                        .await
                        .map_err(|err| Error::Send(err.to_string()))?;
                } else {
                    return Ok(());
                }
            }
            _ = shutdown => {
                // Shutdown the incoming message stream
                let mut message_stream = message_stream.into_inner();
                message_stream.close();
                while message_stream.next().await.is_some() {}

                // Clear out any operations
                operations.lock().await.clear();

                return Ok(());
            }
        }
    }
}

struct ClientInner<GraphqlClient>
where
    GraphqlClient: crate::graphql::GraphqlClient,
{
    #[allow(dead_code)]
    receiver_handle: RemoteHandle<Result<(), Error>>,
    #[allow(dead_code)]
    sender_handle: RemoteHandle<Result<(), Error>>,
    operations: OperationMap<GraphqlClient::Response>,
}

fn json_message<M: WebsocketMessage>(payload: impl serde::Serialize) -> Result<M, Error> {
    Ok(M::new(
        serde_json::to_string(&payload).map_err(|err| Error::Decode(err.to_string()))?,
    ))
}

fn decode_message<T: serde::de::DeserializeOwned, WsMessage: WebsocketMessage>(
    message: WsMessage,
) -> Result<Option<T>, Error> {
    if message.is_ping() || message.is_pong() {
        Ok(None)
    } else if message.is_close() {
        Err(Error::Close(
            message.error_message().unwrap_or(String::new()).to_owned(),
        ))
    } else if let Some(s) = message.text() {
        trace!("Decoding message: {}", s);
        Ok(Some(
            serde_json::from_str::<T>(s).map_err(|err| Error::Decode(err.to_string()))?,
        ))
    } else {
        Ok(None)
    }
}
