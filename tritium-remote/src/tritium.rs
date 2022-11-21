use crate::client::GatewayGraphQLClient;

mod scripts;
mod system;

pub struct Tritium {
    client: GatewayGraphQLClient,
}

impl Tritium {
    pub fn new(client: GatewayGraphQLClient) -> Self {
        Self { client }
    }
}
