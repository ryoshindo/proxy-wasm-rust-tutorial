# Hello World

## wasm build

```shell
$ cargo build --target wasm32-wasi --release
```

## docker build

```shell
docker compose up
```

expected logs

```
[...][wasm] [source/extensions/common/wasm/context.cc:1148] wasm log: Hello, World!
[...][wasm] [source/extensions/common/wasm/context.cc:1148] wasm log: Hello, World!
[...][wasm] [source/extensions/common/wasm/context.cc:1148] wasm log: It's 2023-03-06 16:15:19.328995 UTC, your lucky number is 24.
[...][wasm] [source/extensions/common/wasm/context.cc:1148] wasm log: It's 2023-03-06 16:15:24.329427 UTC, your lucky number is 83.
[...][wasm] [source/extensions/common/wasm/context.cc:1148] wasm log: It's 2023-03-06 16:15:29.329520 UTC, your lucky number is 196.
[...][wasm] [source/extensions/common/wasm/context.cc:1148] wasm log: It's 2023-03-06 16:15:34.329475 UTC, your lucky number is 37.
[...][wasm] [source/extensions/common/wasm/context.cc:1148] wasm log: It's 2023-03-06 16:15:39.329744 UTC, your lucky number is 135.
[...][wasm] [source/extensions/common/wasm/context.cc:1148] wasm log: It's 2023-03-06 16:15:44.329775 UTC, your lucky number is 48.
[...][wasm] [source/extensions/common/wasm/context.cc:1148] wasm log: It's 2023-03-06 16:15:49.332478 UTC, your lucky number is 218.
```
