# http-server

A tiny Rust HTTP server. The HTTP stack is separate from the example apps that use it.

## Layout

```text
src/http/       # HTTP types: request, response, method, status, query string
src/server.rs   # TCP server + Handler trait
src/example/    # Example apps only (website + API)
example/data/   # Example fixtures (e.g. items list)
src/main.rs     # Starts the example apps
```

## Example apps

| App     | Address                 | Notes                                      |
|---------|-------------------------|--------------------------------------------|
| Website | `http://localhost/`     | Port 80                                    |
| API     | `http://127.0.0.1:8081` | Search/pagination over `example/data/items.txt` |

## Running locally

Port 80 needs elevated privileges:

```bash
sudo cargo run
```

Then try:

```bash
curl http://localhost/
curl http://localhost/hello
curl 'http://127.0.0.1:8081/items?q=berry&page=1&limit=2'
```

## CI

GitHub Actions (`.github/workflows/ci.yml`) runs `cargo test --all --locked` on every push/PR targeting `main`.
