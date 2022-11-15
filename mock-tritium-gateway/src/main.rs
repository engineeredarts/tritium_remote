use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use tungstenite::Message;

use serde::Serialize;
use tinytemplate::{format_unescaped, TinyTemplate};

static GRAPHQL_RESPONSE: &str = "GRAPHQL_RESPONSE";

static GRAPHQL_RESPONSE_TEMPLATE: &str = r#"\{
    "type": "graphql_response",
    "request_id:": {request_id},
    "data": { data_json } 
}"#;

#[derive(Serialize)]
struct Context {
    request_id: i32,
    data_json: String,
}

/// A WebSocket echo server
fn main() {
    let address = "127.0.0.1:1234";
    println!("mock-tritium-gateway, listening to {}", address);

    let server = TcpListener::bind(address).unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut tt = TinyTemplate::new();
            tt.set_default_formatter(&format_unescaped);

            tt.add_template(GRAPHQL_RESPONSE, GRAPHQL_RESPONSE_TEMPLATE)
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

                if msg.is_text() {
                    let context = Context {
                        request_id: 123,
                        data_json: r#"{ "hello": "Hello World!" }"#.to_string(),
                    };
                    let reply = tt.render(GRAPHQL_RESPONSE, &context).unwrap();
                    println!("reply: {}", reply);
                    websocket.write_message(Message::text(reply)).unwrap();
                }
            }
        });
    }
}
