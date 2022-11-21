use crate::graphql::GraphQLOperation;
use futures::{
    channel::{mpsc, oneshot},
    future::{Future, RemoteHandle},
    lock::Mutex,
    sink::{Sink, SinkExt},
    stream::{Stream, StreamExt},
    task::SpawnExt,
};
use std::{collections::HashMap, pin::Pin, sync::Arc};

use async_tungstenite::tungstenite::Message;

use super::{
    error::TritiumError,
    graphql::GenericResponse,
    protocol::{MessageFromGateway, MessageToGateway},
    tokio_spawner::TokioSpawner,
};

pub struct GatewayGraphQLClientBuilder {}

impl GatewayGraphQLClientBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn build(
        self,
        url: &str,
        auth_token: &str,
    ) -> Result<GatewayGraphQLClient, TritiumError> {
        let (ws_stream, _) = async_tungstenite::tokio::connect_async(url).await?;

        let (websocket_sink, websocket_stream) = ws_stream.split();

        let operations = Arc::new(Mutex::new(HashMap::new()));
        let (sender_sink, sender_stream) = mpsc::channel::<Message>(1);
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();

        let runtime = TokioSpawner::current();

        let sender_handle = runtime
            .spawn_with_handle(sender_loop(
                sender_stream,
                websocket_sink,
                Arc::clone(&operations),
                shutdown_receiver,
            ))
            .map_err(|err| TritiumError::GenericError(err.to_string()))?;

        let receiver_handle = runtime
            .spawn_with_handle(receiver_loop(
                websocket_stream,
                sender_sink.clone(),
                Arc::clone(&operations),
                shutdown_sender,
            ))
            .map_err(|err| TritiumError::GenericError(err.to_string()))?;

        Ok(GatewayGraphQLClient {
            inner: Arc::new(ClientInner {
                receiver_handle,
                operations,
                sender_handle,
            }),
            sender_sink,
            next_request_id: 0,
            auth_token: auth_token.to_string(),
        })
    }
}

pub struct GatewayGraphQLClient {
    inner: Arc<ClientInner>,
    sender_sink: mpsc::Sender<Message>,
    next_request_id: u64,
    auth_token: String,
}

pub struct PendingGraphQLRequest<Operation: GraphQLOperation> {
    pub id: RequestId,
    pub result: Pin<Box<dyn Future<Output = Result<Operation::Response, Error>> + Send>>,
}

impl GatewayGraphQLClient {
    pub async fn graphql_query<'a, Operation>(
        &mut self,
        operation: Operation,
    ) -> Result<PendingGraphQLRequest<Operation>, Error>
    where
        Operation: GraphQLOperation + Unpin + Send + 'static,
    {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        let (sender, receiver) = mpsc::channel(1);
        self.inner
            .operations
            .lock()
            .await
            .insert(request_id, sender);

        let msg = json_message(MessageToGateway::GraphQL {
            auth_token: &self.auth_token,
            request_id,
            document: operation.get_document(),
            variable_values: operation.get_variables(),
        })
        .map_err(|err| Error::Send(err.to_string()))?;

        // println!("msg: {}", msg);

        self.sender_sink
            .send(msg)
            .await
            .map_err(|err| Error::Send(err.to_string()))?;

        let result = Box::pin(async move {
            let (r, _) = receiver.into_future().await;
            match r {
                Some(Ok(response)) => operation
                    .decode(response)
                    .map_err(|err| Error::Decode(err.to_string())),
                Some(Err(error)) => Err(Error::GatewayError(error)),
                _ => Err(Error::Unknown("no response".to_string())),
            }
        });

        Ok(PendingGraphQLRequest {
            id: request_id,
            result,
        })
    }
}

struct ClientInner {
    #[allow(dead_code)]
    receiver_handle: RemoteHandle<Result<(), Error>>,
    #[allow(dead_code)]
    sender_handle: RemoteHandle<Result<(), Error>>,
    operations: OperationMap,
}

type OperationResponse = Result<GenericResponse, String>;
type OperationSender = mpsc::Sender<OperationResponse>;
type RequestId = u64;
type OperationMap = Arc<Mutex<HashMap<RequestId, OperationSender>>>;

async fn receiver_loop(
    mut receiver: impl Stream<Item = Result<Message, tungstenite::Error>> + Unpin,
    mut sender: mpsc::Sender<Message>,
    operations: OperationMap,
    shutdown: oneshot::Sender<()>,
) -> Result<(), Error> {
    while let Some(msg) = receiver.next().await {
        println!("Received message: {:?}", msg);
        if let Err(err) = handle_message(msg, &mut sender, &operations).await {
            println!("message handler error, shutting down: {err:?}");
            break;
        }
    }

    shutdown
        .send(())
        .map_err(|_| Error::SenderShutdown("Couldn't shutdown sender".to_owned()))
}

async fn handle_message(
    msg: Result<Message, tungstenite::Error>,
    _sender: &mut mpsc::Sender<Message>,
    operations: &OperationMap,
) -> Result<(), Error> {
    let from_gateway = decode_message::<MessageFromGateway<GenericResponse>>(
        msg.map_err(|err| Error::Decode(err.to_string()))?,
    )
    .map_err(|err| Error::Decode(err.to_string()))?;

    let from_gateway = match from_gateway {
        Some(m) => m,
        None => return Ok(()),
    };

    match from_gateway {
        MessageFromGateway::GraphQLResponse {
            request_id,
            data,
            error,
        } => {
            println!("GraphQL response");
            println!("  request id: {}", request_id);
            println!("  data: {:?}", data);

            let mut sink = operations
                .lock()
                .await
                .remove(&request_id) // TODO only remove if no more messages expected
                .ok_or_else(|| Error::Decode("Received message for unknown request".to_owned()))?
                .clone();

            if let Some(d) = data {
                sink.send(Ok(d))
                    .await
                    .map_err(|err| Error::Send(err.to_string()))?
            } else if let Some(e) = error {
                sink.send(Err(e))
                    .await
                    .map_err(|err| Error::Send(err.to_string()))?
            }
        }
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
/// Error type
pub enum Error {
    /// Unknown error
    #[error("unknown: {0}")]
    Unknown(String),
    // /// Custom error
    // #[error("{0}: {1}")]
    // Custom(String, String),
    /// Unexpected close frame
    #[error("got close frame, reason: {0}")]
    Close(String),
    /// Decoding / parsing error
    #[error("message decode error, reason: {0}")]
    Decode(String),
    /// Sending error
    #[error("message sending error, reason: {0}")]
    Send(String),
    // /// Futures spawn error
    // #[error("futures spawn error, reason: {0}")]
    // SpawnHandle(String),
    /// Sender shutdown error
    #[error("sender shutdown error, reason: {0}")]
    SenderShutdown(String),
    /// Binary messages not supported (yet)
    #[error("binary messages not yet supported")]
    BinaryMessagesNotSupported(),
    /// Any error message returned by Gateway
    #[error("{0}")]
    GatewayError(String),
}

impl From<Error> for TritiumError {
    fn from(err: Error) -> TritiumError {
        TritiumError::GenericError(err.to_string())
    }
}

async fn sender_loop(
    message_stream: mpsc::Receiver<Message>,
    mut ws_sender: impl Sink<Message, Error = tungstenite::Error> + Unpin,
    operations: OperationMap,
    shutdown: oneshot::Receiver<()>,
) -> Result<(), Error> {
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

fn json_message(payload: impl serde::Serialize) -> Result<Message, Error> {
    Ok(Message::Text(
        serde_json::to_string(&payload).map_err(|err| Error::Decode(err.to_string()))?,
    ))
}

fn decode_message<T: serde::de::DeserializeOwned>(msg: Message) -> Result<Option<T>, Error> {
    match msg {
        Message::Ping(_) => Ok(None),
        Message::Pong(_) => Ok(None),
        Message::Text(s) => {
            let m = serde_json::from_str::<T>(s.as_ref())
                .map_err(|err| Error::Decode(err.to_string()))?;
            Ok(Some(m))
        }
        Message::Binary(_) => Err(Error::BinaryMessagesNotSupported()),
        Message::Close(frame) => {
            let reason = match frame {
                Some(f) => f.reason.to_string(),
                None => "(unknown reason)".to_string(),
            };
            Err(Error::Close(reason))
        }
        _ => Ok(None),
    }
}
