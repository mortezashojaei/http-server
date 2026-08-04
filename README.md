# http-server

A tiny Rust HTTP server experiment with two sibling HTTP services:

- Website on `127.0.0.1:80` (`http://localhost/`)
- API on `127.0.0.1:8081`

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
