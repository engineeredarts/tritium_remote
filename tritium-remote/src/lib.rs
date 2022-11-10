use tokio_tungstenite::connect_async;

pub struct Connection {
    // pub url: String,
    pub open: bool,
}

pub async fn connect(url: &str) -> Connection {
    let (_ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    Connection {
        // url: String::from(url),
        open: true,
    }
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
