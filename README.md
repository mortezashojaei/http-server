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
| Website | `http://localhost/`     | Port 80; `POST /echo` returns the request body |
| API     | `http://127.0.0.1:8081` | `GET /items` search/pagination; `POST /items` adds an item from the body |

## Running locally

Port 80 needs elevated privileges:

```bash
sudo cargo run
```

Then try:

```bash
curl http://localhost/
curl http://localhost/hello
curl -d 'ping' http://localhost/echo
curl 'http://127.0.0.1:8081/items?q=berry&page=1&limit=2'
curl -d 'dragonfruit' http://127.0.0.1:8081/items
```

## CI

GitHub Actions (`.github/workflows/ci.yml`) runs `cargo test --all --locked` on every push/PR targeting `main`.
