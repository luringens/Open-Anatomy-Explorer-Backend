# INF319 Backend

## Configuration

This backend must be configured through environment variables. For example:

```txt
MODELS_DIR=./models
CORS=http://example.com
DATABASE_URL=db.sqlite
```

If a different file than `db.sqlite` is wanted, make sure to reflect this in the provided
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
