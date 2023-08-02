# tritium_remote

A library for interacting with Tritium systems.

-   Connects to a running Tritium system via the Gateway node, initially by WebSocket only
-   Uses GraphQL to send commands and request data

Implemented in Rust but with bindings to

-   Python

_NB_ This library is currently only published and tested on linux. However the rust crate ought to work on other platforms.

_NB_ **This is only to be used on a trusted local network.** The Tritium system will be accessed over LAN without encryption.

[Hosted docs are here](https://tritiumrobot.cloud/docs/remote/) (require login).

## APIs

### Sequence playback

-   Start & stop sequence

### Scripting

-   Start & stop scripts
-   Send and receive arbitrary messages via bidirectional byte stream

## Running Examples

The Rust and Python examples use the _TRITIUM_AUTH_TOKEN_ environment variable to get the JWT access token they need.

Pending a proper UI for generating tokens, the best way for now is...

1. Log into the [Tritium cloud UI](https://tritiumrobot.cloud/)
2. Connect to your locally-running system
3. Go to the "Scripts" page
4. Select the API Keys tab
5. Click "CREATE AN API KEY"

```bash
$ export TRITIUM_AUTH_TOKEN={paste x-tritium-auth contents here}
$ cd tritium-remote/examples/system_info
$ cargo run
```

## Generated GraphQL Code

Rust structures are generated from the GraphQL schemas using [graphql-client](https://crates.io/crates/graphql_client_cli)

Install _graphql-client_ with:

```bash
$ cargo install graphql_client_cli
```

Then to generate Rust code for the _playSequence_ mutation as an example, in the repository root:

```bash
$ cd tritium-remote
$ graphql-client generate --schema-path graphql/schemas/tritium.graphql --output-directory src/graphql/mutations graphql/mutations/play_sequence.graphql
```

## License
This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
