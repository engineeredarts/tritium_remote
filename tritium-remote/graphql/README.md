# Generated GraphQL Code

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
