use async_tungstenite::tokio::connect_async;
use async_tungstenite::tungstenite::protocol::Message;
use futures::StreamExt;
use futures_util::sink::Sink;
use futures_util::SinkExt;

pub struct Connection {
    sink: Box<dyn Sink<Message, Error = tungstenite::error::Error>>,
}

pub async fn connect(url: &str) -> Connection {
    println!("[tritium-remote] connecting to {}...", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("[tritium-remote] CONNECTED");

    let (sink, _stream) = ws_stream.split();

    Connection {
        sink: Box::new(sink),
    }
}

#[derive(Debug, Clone)]
pub struct TritiumError;

pub async fn do_something(connection: &Connection) -> Result<(), TritiumError> {
    println!("[tritium-remote] do_something");
    send(connection, "do_something")?;
    Ok(())
}

fn send(connection: &Connection, text: &str) -> Result<(), TritiumError> {
    let m = Message::text("do_something");

    Box::pin(connection.sink).send(m);

    Ok(())
}
