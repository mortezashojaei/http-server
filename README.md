# http-server

A tiny Rust HTTP server experiment. Right now it binds to `127.0.0.1:8080` and runs the custom server implementation under `src/server.rs` and `src/http`.

## Running locally

```bash
cargo run
```

That will build the project and start the server. Adjust the bind address in `src/main.rs` if you need to listen on another interface or port.
