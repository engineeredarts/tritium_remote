mod client;
mod websockets;

mod error;
use error::TritiumError;

// use tungstenite;
use async_tungstenite;
use futures_util::StreamExt;
use tokio;
use tungstenite::protocol::Message;

use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub struct Connection {
    sender: mpsc::UnboundedSender<Message>,
}

pub async fn connect(url: &str) -> Connection {
    println!("[tritium-remote] connecting to {}...", url);
    let (ws_stream, _) = async_tungstenite::tokio::connect_async(url)
        .await
        .expect("Failed to connect");
    println!("[tritium-remote] CONNECTED");

    let (sink, _stream) = ws_stream.split();
    let (send_channel_tx, send_channel_rx) = mpsc::unbounded_channel::<Message>();
    let send_channel_rx_stream = UnboundedReceiverStream::new(send_channel_rx);

    tokio::spawn(send_channel_rx_stream.map(Ok).forward(sink));

    Connection {
        sender: send_channel_tx,
    }
}

pub async fn do_something(connection: &Connection) -> Result<(), TritiumError> {
    println!("[tritium-remote] do_something");
    send(connection, r#"{ "type": "graphql", "request_id": 123 }"#).await?;
    Ok(())
}

async fn send(connection: &Connection, text: &str) -> Result<(), TritiumError> {
    let m = Message::text(text);

    connection.sender.send(m)?;

    Ok(())
}
