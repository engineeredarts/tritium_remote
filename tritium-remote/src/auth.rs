use std::env;

use crate::error::TritiumError;

const TRITIUM_AUTH_TOKEN: &str = "TRITIUM_AUTH_TOKEN";

pub fn get_tritium_auth_token() -> Result<String, TritiumError> {
    env::var(TRITIUM_AUTH_TOKEN)
        .map_err(|_| TritiumError::AuthenticationError("TRITIUM_AUTH_TOKEN not set".to_string()))
}
