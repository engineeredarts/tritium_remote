use crate::client::GatewayGraphQLClient;

pub mod scripts;
pub mod sequences;
pub mod system;

/// A remote Tritium system, connected via unencrypted WebSocket over the LAN.
///
/// The Tritium object has methods for interacting with the system, such as...
/// * Querying [system information](Tritium::query_basic_system_info)
/// * [Starting](Tritium::start_script) and [stopping](Tritium::stop_script) scripts running on the system
/// * Playing pre-animated [sequences](Tritium::play_sequence)
pub struct Tritium {
    /// The connection to the remote system.
    client: GatewayGraphQLClient,
}

impl Tritium {
    /// Creates and returns a new Tritium instance.
    pub fn new(client: GatewayGraphQLClient) -> Self {
        Self { client }
    }
}
