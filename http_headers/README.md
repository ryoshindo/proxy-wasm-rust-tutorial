# HTTP Headers

## wasm build

```shell
$ cargo build --target wasm32-wasi --release
```

## docker build

```shell
docker compose up
```

execute this shell

```shell
curl localhost:10000/hello
```

expected logs

```
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 -> :authority: localhost:10000
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 -> :path: /hello
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 -> :method: GET
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 -> :scheme: http
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 -> user-agent: curl/7.85.0
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 -> accept: */*
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 -> x-forwarded-proto: http
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 -> x-request-id: e85a745b-8c40-45ce-8559-27eafc3ce0c5
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 <- :status: 200
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 <- hello: World
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 <- powered-by: proxy-wasm
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 <- content-length: 14
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 <- content-type: text/plain
[wasm] [source/extensions/common/wasm/context.cc:1148] wasm log http_headers: #2 completed.
```
