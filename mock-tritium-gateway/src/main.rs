use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use tungstenite::Message;

use serde::Serialize;
use serde_json::Value;
use tinytemplate::{format_unescaped, TinyTemplate};

// static GRAPHQL: &str = "graphql";
static GRAPHQL_RESPONSE: &str = "graphql_response";

static GRAPHQL_RESPONSE_TEMPLATE: &str = r#"\{
    "type": "graphql_response",
    "request_id:": {request_id},
    "data": { data_json } 
}"#;

#[derive(Serialize)]
struct ResponseContext {
    request_id: u64,
    data_json: String,
}

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
                    let request_json = msg.into_text().unwrap();
                    let request: Value = serde_json::from_str(&request_json).unwrap();

                    let request_type = request["type"].as_str().unwrap();
                    println!("  request type: {}", request_type);
                    match request_type {
                        "graphql" => {
                            let request_id = request["request_id"].as_u64().unwrap();
                            println!("  request id: {}", request_id);

                            let context = ResponseContext {
                                request_id,
                                data_json: r#"{ "hello": "Hello World!" }"#.to_string(),
                            };
                            let response = tt.render(GRAPHQL_RESPONSE, &context).unwrap();
                            println!("response: {}", response);
                            websocket.write_message(Message::text(response)).unwrap();
                        }
                        _ => {
                            println!("(unhandled request type: {})", request_type);
                        }
                    }
                }
            }
        });
    }
}
