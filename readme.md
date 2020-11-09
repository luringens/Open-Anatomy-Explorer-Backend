# INF319 Backend

## Config

This backend must be configured through environment variables. For example:

```txt
LABEL_DATA_DIR=./data-labels
QUIZ_DATA_DIR=./data-quiz
```

## Building

To get live reloading, run:

```sh
systemfd --no-pid -s http::3000 -- cargo watch -x run
```
