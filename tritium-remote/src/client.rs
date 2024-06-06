use futures::{
    channel::{mpsc, oneshot},
    future::Future,
    lock::Mutex,
    sink::{Sink, SinkExt},
    stream::{Stream, StreamExt},
};
use log::{trace, warn};
use std::{collections::HashMap, pin::Pin, sync::Arc};
use tokio::task::JoinHandle;

use async_tungstenite::tungstenite::client::IntoClientRequest;
use async_tungstenite::tungstenite::Message;

use crate::{
    error::TritiumError,
    graphql::{GenericResponse, GenericSubscription, GraphQLOperation},
    protocol::{MessageFromGateway, MessageToGateway},
    tritium,
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
        description: Option<String>,
    ) -> Result<GatewayGraphQLClient, TritiumError> {
        // GET request with websocket headers
        let mut request = url
            .into_client_request()
            .map_err(|err| TritiumError::GenericError(err.to_string()))?;

        let headers = request.headers_mut();

        // add auth token header
        headers.insert("x-tritium-token", auth_token.parse().unwrap());

        // add metadata header
        let metadata = tritium::session::Metadata {
            session_type: "graphql",
            description,
        };
        let metadata_json = serde_json::to_string(&metadata).unwrap();
        headers.insert("x-tritium-session-metadata", metadata_json.parse().unwrap());

        let (ws_stream, _) = async_tungstenite::tokio::connect_async(request).await?;

        let (websocket_sink, websocket_stream) = ws_stream.split();

        let operations = Arc::new(Mutex::new(HashMap::new()));

        let (sender_sink, sender_stream) = mpsc::channel::<Message>(1);
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();

        let sender_handle = tokio::spawn(sender_loop(
            sender_stream,
            websocket_sink,
            shutdown_receiver,
        ));

        let receiver_handle = tokio::spawn(receiver_loop(
            websocket_stream,
            operations.clone(),
            shutdown_sender,
        ));

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

pub type GenericVariables = serde_json::Value;

pub struct PendingGenericGraphQLRequest {
    pub id: RequestId,
    pub result: Pin<Box<dyn Future<Output = Result<GenericResponse, Error>> + Send>>,
}

pub struct PendingGenericGraphQLSubscription {
    pub id: RequestId,
    pub result: Pin<Box<dyn Future<Output = Result<GenericSubscription, Error>> + Send>>,
}

impl GatewayGraphQLClient {
    /// Static, strongly-typed query (document must be known at compile time)
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
        let op = OperationResponseHandler {
            response_tx: sender,
            expect_more: false,
        };
        self.inner.operations.lock().await.insert(request_id, op);

        let msg = json_message(MessageToGateway::GraphQL {
            auth_token: &self.auth_token,
            request_id,
            document: operation.get_document(),
            variable_values: operation.get_variables(),
        })
        .map_err(|err| Error::Send(err.to_string()))?;

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

    /// Generic, non-strongly typed query (document may be created dynamically at runtime)
    pub async fn generic_graphql_query<'a>(
        &mut self,
        document: &str,
        variables: GenericVariables,
    ) -> Result<PendingGenericGraphQLRequest, Error> {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        let (sender, receiver) = mpsc::channel(1);
        let op = OperationResponseHandler {
            response_tx: sender,
            expect_more: false,
        };
        self.inner.operations.lock().await.insert(request_id, op);

        let msg = json_message(MessageToGateway::GraphQL {
            auth_token: &self.auth_token,
            request_id,
            document: document,
            variable_values: variables,
        })
        .map_err(|err| Error::Send(err.to_string()))?;

        self.sender_sink
            .send(msg)
            .await
            .map_err(|err| Error::Send(err.to_string()))?;

        let result = Box::pin(async move {
            let (r, _) = receiver.into_future().await;
            match r {
                Some(Ok(response)) => Ok(response),
                Some(Err(error)) => Err(Error::GatewayError(error)),
                _ => Err(Error::Unknown("no response".to_string())),
            }
        });

        Ok(PendingGenericGraphQLRequest {
            id: request_id,
            result,
        })
    }

    /// Generic, non-strongly typed subscription (document may be created dynamically at runtime)
    pub async fn generic_graphql_subscription<'a>(
        &mut self,
        document: &str,
        variables: GenericVariables,
    ) -> Result<PendingGenericGraphQLSubscription, Error> {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        let (sender, receiver) = mpsc::channel(1);
        let op = OperationResponseHandler {
            response_tx: sender,
            expect_more: true,
        };
        self.inner.operations.lock().await.insert(request_id, op);

        let msg = json_message(MessageToGateway::GraphQL {
            auth_token: &self.auth_token,
            request_id,
            document,
            variable_values: variables,
        })
        .map_err(|err| Error::Send(err.to_string()))?;

        self.sender_sink
            .send(msg)
            .await
            .map_err(|err| Error::Send(err.to_string()))?;

        let result = Box::pin(async move {
            let (r, stream) = receiver.into_future().await;
            match r {
                Some(Ok(response)) => {
                    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

                    let _ = tx.send(response);

                    let mut stream = stream;
                    tokio::spawn(async move {
                        loop {
                            let (r, s) = stream.into_future().await;
                            stream = s;

                            match r {
                                Some(Ok(r)) => {
                                    if tx.send(r).is_err() {
                                        break;
                                    }
                                }
                                _ => {
                                    // ?
                                    break;
                                }
                            }
                        }
                    });

                    let subscription = GenericSubscription { results: rx };
                    Ok(subscription)
                }
                Some(Err(error)) => Err(Error::GatewayError(error)),
                _ => Err(Error::Unknown("no response".to_string())),
            }
        });

        Ok(PendingGenericGraphQLSubscription {
            id: request_id,
            result,
        })
    }
}

struct ClientInner {
    #[allow(dead_code)]
    receiver_handle: JoinHandle<Result<(), Error>>,
    #[allow(dead_code)]
    sender_handle: JoinHandle<Result<(), Error>>,
    operations: OperationMap,
}

type RequestId = u64;
type OperationResponse = Result<GenericResponse, String>;
type OperationMap = Arc<Mutex<HashMap<RequestId, OperationResponseHandler>>>;

#[derive(Clone)]
struct OperationResponseHandler {
    response_tx: mpsc::Sender<OperationResponse>,
    expect_more: bool,
}

async fn receiver_loop(
    mut receiver: impl Stream<Item = Result<Message, tungstenite::Error>> + Unpin,
    operations: OperationMap,
    shutdown: oneshot::Sender<()>,
) -> Result<(), Error> {
    while let Some(msg) = receiver.next().await {
        trace!("Received message: {:?}", msg);
        if let Err(err) = handle_message(msg, &operations).await {
            warn!("message handler error: {err:?}");
        }
    }

    shutdown
        .send(())
        .map_err(|_| Error::SenderShutdown("Couldn't shutdown sender".to_owned()))
}

async fn handle_message(
    msg: Result<Message, tungstenite::Error>,
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
            trace!("GraphQL response");
            trace!("  request id: {}", request_id);
            trace!("  data: {:?}", data);

            let ops = operations.lock().await;

            let op = ops
                .get(&request_id) // TODO only remove if no more messages expected
                .ok_or_else(|| Error::Decode("Received message for unknown request".to_owned()))?;

            // TODO support both data and errors in response?
            let mut tx = op.response_tx.clone();
            if let Some(d) = data {
                // pass data on to operation
                tx.send(Ok(d))
                    .await
                    .map_err(|err| Error::Send(err.to_string()))?
            } else if let Some(e) = error {
                // ...or any error
                tx.send(Err(e))
                    .await
                    .map_err(|err| Error::Send(err.to_string()))?
            }

            // operation complete if query or mutation
            if !op.expect_more {
                let mut ops = ops;
                ops.remove(&request_id);
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
    /// Unexpected close frame
    #[error("unexpected close frame, reason: {0}")]
    Close(String),
    /// Decoding / parsing error
    #[error("message decode error: {0}")]
    Decode(String),
    /// Encoding error
    #[error("message encode error: {0}")]
    Encode(String),
    /// Sending error
    #[error("message send error: {0}")]
    Send(String),
    /// Sender shutdown error
    #[error("sender shutdown error: {0}")]
    SenderShutdown(String),
    /// Binary messages not supported (yet)
    #[error("binary messages not supported")]
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
    shutdown: oneshot::Receiver<()>,
) -> Result<(), Error> {
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

                return Ok(());
            }
        }
    }
}

fn json_message(payload: impl serde::Serialize) -> Result<Message, Error> {
    Ok(Message::Text(
        serde_json::to_string(&payload).map_err(|err| Error::Encode(err.to_string()))?,
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
