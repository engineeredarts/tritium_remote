use futures::StreamExt;
mod client;
mod protocol;
mod tokio_spawner;

mod error;
use error::TritiumError;

use async_tungstenite;

use client::{GatewayGraphQLClient, GatewayGraphQLClientBuilder};

pub struct Connection {
    client: GatewayGraphQLClient,
}

pub async fn connect(url: &str) -> Connection {
    let (ws_stream, _) = async_tungstenite::tokio::connect_async(url)
        .await
        .expect("Failed to connect"); 

    let (sink, stream) = ws_stream.split();

    let client = GatewayGraphQLClientBuilder::new()
        .build(stream, sink)
        .await
        .unwrap();

    Connection {
        client, 
    }
}

pub async fn hello_world(connection: &mut Connection) -> Result<(), TritiumError> {
    println!("[tritium-remote] hello_world");
    // send(connection, r#"{ "type": "graphql", "request_id": 123 }"#).await?;

    let query = connection.client.graphql_query().await.unwrap();

    let result = query.result.await;
    println!("result {:?}", result);

    Ok(())
}

