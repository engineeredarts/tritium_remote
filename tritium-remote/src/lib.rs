mod error;
// use futures_util::Sink;
use error::TritiumError;


// use tungstenite;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};

// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::select;
// use tokio::sync::mpsc;
// use tokio_stream::wrappers::UnboundedReceiverStream;

pub struct Connection {
    // sender: mpsc::UnboundedSender<Message>
}

// struct ConnectInner {
//     send_channel_rx: mpsc::UnboundedReceiver<Message>
// }


pub async fn connect(url: &str) -> Connection {
    println!("[tritium-remote] connecting to {}...", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("[tritium-remote] CONNECTED");

    let (mut sink, _stream) = ws_stream.split();
    // let (send_channel_tx, send_channel_rx) = mpsc::unbounded_channel::<Message>();
    // let send_channel_rx_stream = UnboundedReceiverStream::new(send_channel_rx);
    // let send_to_ws = send_channel_rx_stream.map(Ok).forward(sink);


//    tokio::spawn(loop_forever(send_channel_rx, sink)); 

    sink.send(Message::text("hello")).await.unwrap();

    Connection {
        // sender:send_channel_tx
    }
} 

// async fn loop_forever(mut send_channel_rx:mpsc::UnboundedReceiver<Message>, sink: &dyn Sink<Message, Error=tungstenite::error::Error>) {
//     loop {
//         let m = send_channel_rx.recv().await;
//         sink.send(m).await;
    
//     }
// }

pub async fn do_something(connection: &Connection) -> Result<(), TritiumError> {
    println!("[tritium-remote] do_something");
    send(connection, "do_something")?;
    Ok(())
}

fn send(_connection: &Connection, _text: &str) -> Result<(), TritiumError> {
    // let m = Message::text("do_something");

    // connection.sender.send(m).map_err(|e| TritiumError::from(e))?;

    Ok(())
}
