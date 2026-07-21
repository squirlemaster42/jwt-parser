# jwt-parser

A simple CLI tool to decode and validate JWTs signed with HS256. I'm bad at Rust, tell me what I did wrong.

## Prerequisites

- [Rust](https://rustup.rs/)

## Usage

```bash
# Run directly
cargo run -- <JWT_TOKEN> <SECRET>

# Or build and run
cargo build --release
./target/release/jwt-parser <JWT_TOKEN> <SECRET>
```

The tool prints the decoded header, payload, and whether the signature is valid.

### Example

```bash
cargo run -- eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.KMUFsIDTnFmyG3nMiGM6H9FNFUROf3wh7SmqJp-QV30 "a-string-secret-at-least-256-bits-long"
```

## Tests

```bash
cargo test
```
