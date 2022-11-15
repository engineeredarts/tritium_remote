use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use tungstenite::Message;

use serde::Serialize;
use tinytemplate::TinyTemplate;

static HELLO_WORLD: &str = "hello_world";

static HELLO_WORLD_RESPONSE_TEMPLATE: &str = r#"\{
    "type": "graphql_response",
    "request_id:": {request_id},
    "data": \{
        "hello": "Hello World!"
    } 
}"#;

#[derive(Serialize)]
struct Context {
    request_id: i32,
}

/// A WebSocket echo server
fn main() {
    let address = "127.0.0.1:1234";
    println!("mock-tritium-gateway, listening to {}", address);

    let server = TcpListener::bind(address).unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut tt = TinyTemplate::new();
            tt.add_template(HELLO_WORLD, HELLO_WORLD_RESPONSE_TEMPLATE)
                .unwrap();

            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = match websocket.read_message() {
                    Ok(msg) => msg,
                    Err(e) => {
                        println!("{}", e);
                        break;
                    }
                };

                println!("msg: {}", msg);

                // We do not want to send back ping/pong messages.
                // if msg.is_binary() || msg.is_text() {
                //     websocket.write_message(msg).unwrap();
                // }

                if msg.is_text() {
                    let context = Context { request_id: 123 };
                    let reply = tt.render(HELLO_WORLD, &context).unwrap();
                    println!("reply: {}", reply);
                    websocket.write_message(Message::text(reply)).unwrap();
                }
            }
        });
    }
}
