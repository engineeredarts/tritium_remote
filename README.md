# tritium_remote

A library for interacting with Tritium systems.

- Connects to a running Tritium system via the Gateway node, initially by WebSocket only
- Uses GraphQL to send commands and request data

Implemented in Rust but with bindings to

- Python
- C/C++

## APIs

### Sequence playback

- Start & stop sequence

### Scripting

- Start & stop scripts
- Send and receive arbitrary messages via bidirectional byte stream
