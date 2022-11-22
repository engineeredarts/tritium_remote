use crate::client::GatewayGraphQLClient;

pub mod scripts;
pub mod sequences;
pub mod system;

/// A remote Tritium system, connected via unencrypted WebSocket over the LAN.
pub struct Tritium {
    /// The connection to the remote system
    client: GatewayGraphQLClient,
}

impl Tritium {
    /// Creates and returns a new Tritium instance.
    pub fn new(client: GatewayGraphQLClient) -> Self {
        Self { client }
    }
}
