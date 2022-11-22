# tritium_remote

A library for interacting with Tritium systems.

-   Connects to a running Tritium system via the Gateway node, initially by WebSocket only
-   Uses GraphQL to send commands and request data

Implemented in Rust but with bindings to

-   Python
-   C/C++

_NB_ It is assumed that the Tritium system is being accessed over a LAN without encryption.
The Gateway node must be launched with the _--insecure-websockets_ option, like so:

```
ExecStart = /opt/tritium/bin/gateway_node --insecure-websockets
```

## APIs

### Sequence playback

-   Start & stop sequence

### Scripting

-   Start & stop scripts
-   Send and receive arbitrary messages via bidirectional byte stream

## Running Examples

The Rust and Python examples use the _TRITIUM_AUTH_TOKEN_ environment variable to get the JWT access token they need.

Pending a proper UI for generating tokens, the best way for now is...

1. Log into the [Tritium cloud UI](https://develop.tritiumrobot-test.cloud/)
2. Connect to your locally-running system
3. Open the browser console -> _Application_ tab -> _Cookies_
4. Select on _x-tritium-auth_ and copy the contents

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
