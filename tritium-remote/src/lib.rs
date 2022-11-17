use futures::StreamExt;
mod client;
mod protocol;
mod tokio_spawner;

mod auth;
mod graphql;

mod error;
use error::TritiumError;

use async_tungstenite;

use client::{GatewayGraphQLClient, GatewayGraphQLClientBuilder};

use graphql::basic_system_info::basic_system_info::BasicSystemInfoSystem;
use graphql::basic_system_info::BasicSystemInfo;
use graphql::QueryOperation;

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

    Connection { client }
}

// pub async fn hello_world(connection: &mut Connection) -> Result<(), TritiumError> {
//     println!("[tritium-remote] hello_world");
//     // send(connection, r#"{ "type": "graphql", "request_id": 123 }"#).await?;

//     let query = connection.client.graphql_query().await.unwrap();

//     let result = query.result.await;
//     println!("result {:?}", result);

//     Ok(())
// }

pub async fn query_basic_system_info(
    connection: &mut Connection,
) -> Result<BasicSystemInfoSystem, TritiumError> {
    let operation = QueryOperation::<BasicSystemInfo>::new(
        graphql::basic_system_info::basic_system_info::Variables {},
    );
    let query = connection.client.graphql_query(operation).await.unwrap();

    let response = query
        .result
        .await
        .map_err(|err| TritiumError::GenericError(err.to_string()))?;

    match response.data {
        Some(data) => Ok(data.system),
        _ => Err(TritiumError::GenericError("no data".to_string())),
    }
}
