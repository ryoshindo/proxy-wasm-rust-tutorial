# HTTP Body

## wasm build

```shell
$ cargo build --target wasm32-wasi --release
```

## docker build

```shell
docker compose up
```

### Response without secrets.

Send HTTP request to localhost:10000/hello:

```shell
$ curl localhost:10000/hello
Everyone may read this message.
```

### Response with (redacted) secrets.

Send HTTP request to localhost:10000/secret:

```shell
$ curl localhost:10000/secret
Original message body (50 bytes) redacted.
```
