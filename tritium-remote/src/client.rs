use futures::{
    channel::{mpsc, oneshot},
    future::RemoteHandle,
    sink::{Sink, SinkExt},
    stream::{Stream, StreamExt},
    lock::Mutex
};
use std::{collections::HashMap, sync::Arc};
use tokio;
use uuid::Uuid; 
 
use async_tungstenite::tungstenite::Message;

// use super::websockets::WebsocketMessage;

pub struct GatewayGraphQLClientBuilder {}

impl GatewayGraphQLClientBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn build(
        self,
        mut websocket_stream: impl Stream<Item = Result<Message, tungstenite::Error>>
            + Unpin
            + Send
            + 'static,
        mut websocket_sink: impl Sink<tungstenite::Message, Error = tungstenite::Error>
            + Unpin
            + Send
            + 'static,
    ) -> GatewayGraphQLClient {
        let (mut sender_sink, _sender_stream) = mpsc::channel::<Message>(1);
 
        // start receiving & processing messages
        // tokio::spawn(receiver_loop(websocket_stream, sender_sink));

        GatewayGraphQLClient {}
    } 
}

pub struct GatewayGraphQLClient {}

type GenericResponse = graphql_client::Response<serde_json::Value>;
type OperationSender = mpsc::Sender<GenericResponse>;
type OperationMap = Arc<Mutex<HashMap<Uuid, OperationSender>>>;

async fn receiver_loop(
    mut receiver: impl Stream<Item = Result<Message, tungstenite::Error>> + Unpin,
    mut sender: mpsc::Sender<Message>,
    operations: OperationMap,
    shutdown: oneshot::Sender<()>,
) -> Result<(), Error>
{
    while let Some(msg) = receiver.next().await {
        println!("Received message: {:?}", msg);
        if let Err(err) =
            handle_message(msg, &mut sender, &operations).await
        {
            println!("message handler error, shutting down: {err:?}");
            break;
        }
    }

    shutdown
        .send(())
        .map_err(|_| Error::SenderShutdown("Couldn't shutdown sender".to_owned()))
}

async fn handle_message(msg:Result<Message, tungstenite::Error>, sender: &mut mpsc::Sender<Message>, operations: &OperationMap) -> Result<(), Error> {
    // TODO
    Ok(())
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
    /// Binary messages not supported (yet)
    #[error("binary messages not yet supported")]
    BinaryMessagesNotSupported(),
}

async fn sender_loop(
    message_stream: mpsc::Receiver<Message>,
    mut ws_sender: impl Sink<Message, Error=tungstenite::Error> + Unpin,
    operations: OperationMap,
    shutdown: oneshot::Receiver<()>,
) -> Result<(), Error>
{
    use futures::{future::FutureExt, select};

    let mut message_stream = message_stream.fuse();
    let mut shutdown = shutdown.fuse();

    loop {
        select! {
            msg = message_stream.next() => {
                if let Some(msg) = msg {
                    println!("Sending message: {:?}", msg);
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

// struct ClientInner<GraphqlClient>
// where
//     GraphqlClient: crate::graphql::GraphqlClient,
// {
//     #[allow(dead_code)]
//     receiver_handle: RemoteHandle<Result<(), Error>>,
//     #[allow(dead_code)]
//     sender_handle: RemoteHandle<Result<(), Error>>,
//     operations: OperationMap<GraphqlClient::Response>,
// }

struct ClientInner {
}

fn json_message(payload: impl serde::Serialize) -> Result<Message, Error> {
    Ok(Message::Text(
        serde_json::to_string(&payload).map_err(|err| Error::Decode(err.to_string()))?,
    ))
}

fn decode_message<T: serde::de::DeserializeOwned>(
    msg: Message,
) -> Result<Option<T>, Error> {
    match msg {
        Message::Ping(_) => Ok(None),
        Message::Pong(_) => Ok(None),
        Message::Text(s) => {
            let m = serde_json::from_str::<T>(s.as_ref()).map_err(|err| Error::Decode(err.to_string()))?;
            Ok(Some(m))
        },
        Message::Binary(_) => Err(Error::BinaryMessagesNotSupported()),
        Message::Close(frame) => {
            let reason = match frame {
                Some(f) => f.reason.to_string(),
                None => "(unknown reason)".to_string()
            };
            Err(Error::Close(reason))
        }
        _ => Ok(None)
    }
}
