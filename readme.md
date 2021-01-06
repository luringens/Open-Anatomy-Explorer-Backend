# INF319 Backend

## Configuration

This backend must be configured through environment variables. For example:

```txt
MODELS_DIR=./models
CORS=^https?:\/\/(\w+\.)*example\.com
DATABASE_URL=db.sqlite
```

- Note that CORS is a regex string. It should be anchored by beginning with `^` to avoid
  maliciousness. See the warning and spec in the
  [Rocket documentation](https://docs.rs/rocket_cors/*/rocket_cors/type.AllowedOrigins.html) for
  details.
  - It will always allow CORS from `http(s)://localhost:xxxx` for testing purposes.
- If a different file than `db.sqlite` is wanted, make sure to reflect this in the provided
  `rocket.toml`.

### Database

This application requires a SQLite database to store data in. This is bundled on build-time, but it
still needs to be set up. This can be done with [`diesel_cli`](https://diesel.rs/) as such:

```sh
# Installation
cargo install diesel_cli --no-default-features --features "sqlite-bundled"

# Configuration: either put a DATABASE_URL in `.env` as above, put it in an environment variable, or
diesel setup --database-url="db.sqlite"

# Set up SQLite
diesel migration run
```
