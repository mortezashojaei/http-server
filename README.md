# http-server

A tiny Rust HTTP server experiment. It binds to `127.0.0.1:8080`, parses raw TCP bytes into HTTP requests, then feeds them through a pluggable `Handler` that returns structured HTTP responses (`StatusCode` + body).

## Running locally

```bash
cargo run
```

That will build the project and start the server. Adjust the bind address in `src/main.rs` if you need to listen on another interface or port.

The default `WebsiteHandler` renders three routes:

- `GET /` – simple HTML welcome message
- `GET /health` – plain-text health probe
- everything else → `404 Not Found`

Extend `WebsiteHandler` (or implement your own `Handler`) to add more routes.

## Tests & CI

Unit tests live alongside the HTTP modules. Run them with:

```bash
cargo test
```

GitHub Actions (`.github/workflows/ci.yml`) runs the same test suite on every push/PR targeting `main`, so merges require a green test run.
