# http-server

A tiny Rust HTTP server experiment. Right now it binds to `127.0.0.1:8080` and runs the custom server implementation under `src/server.rs` and `src/http`.

## Running locally

```bash
cargo run
```

That will build the project and start the server. Adjust the bind address in `src/main.rs` if you need to listen on another interface or port.

## Tests & CI

Unit tests live alongside the HTTP parsing modules. Run them with:

```bash
cargo test
```

GitHub Actions (`.github/workflows/ci.yml`) runs the same test suite on every push/PR targeting `main`, so merges require a green test run.
