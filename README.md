# Progrs

`progrs` (prog-R-S), is a wasm binary that gives you a progress bar for the current day, week, year.

### Setup

```shell
curl https://get.wasmer.io -sSfL | sh # Install wasmer runtime
cargo install cargo-wasix
cargo wasix build # or cargo wasix build --release
```

This will build a `wasm` binary in `target/wasm32-wasmer-wasi/debug/progrs.wasm`. You can run this binary with `wasmer run`.