# tritium_remote

A library for interacting with Tritium systems. [Hosted docs are here](https://docs.engineeredarts.co.uk/en/user/tritium_remote) (require login).

- Connects to a running Tritium system by WebSocket
- Uses GraphQL to send commands and request data

Implemented in Rust with bindings to other languages:

- [Python](https://pypi.org/project/tritium-remote/) *NB* **DEPRECATED** in favour of the pure Python `tritium-remote-py`

This library is currently only published and tested on Linux. However the rust crate ought to work on other platforms.

**This is only to be used on a trusted local network.** The Tritium system will be accessed over LAN without encryption.

## APIs

### Sequence playback

- Start & stop sequence

### Scripting

- Start & stop scripts
- Post messages to named channels to which scripts may subscribe

### Generic Queries and Mutations

- Execute user-supplied GraphQL query or mutation documents
- Optional variables, supplied as JSON
- Any response is returned as a JSON encoded object

## Running Examples

The Rust and Python examples use the _TRITIUM_AUTH_TOKEN_ environment variable to get the JWT access token they need.

To generate an access token...

1. Log into the [Tritium cloud UI](https://tritiumrobot.cloud/)
2. Connect to your locally-running system
3. Go to the "Scripts" page
4. Select the API Keys tab
5. Click "CREATE AN API KEY"

```bash
$ export TRITIUM_AUTH_TOKEN="{paste token here}"
$ cargo run --example system_info
```

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
