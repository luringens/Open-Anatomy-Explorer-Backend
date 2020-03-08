# INF319 Backend

## Config

This backend must be configured through environment variables. For example:

```txt
RUST_LOG=inf319-backend=info,actix=info
HOST=localhost
CORS_ACCEPT=http://localhost:8000
PORT=5000
DATA_DIR=./json
```

## Building

To get live reloading, run:

```sh
systemfd --no-pid -s http::3000 -- cargo watch -x run
```
