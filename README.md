# Progrs

`progrs` (prog-R-S), is a wasm binary that gives you a progress bar for the current day, week, year.

### Setup

```shell
curl https://get.wasmer.io -sSfL | sh # Install wasmer runtime
cargo install cargo-wasix
cargo wasix build # or cargo wasix build --release
```

This will build a `wasm` binary in `target/wasm32-wasmer-wasi/debug/progrs.wasm`. You can run this binary with `wasmer run`.

### Thank you to

 - [chrono](https://github.com/chronotope/chrono)
 - [indicatif](https://github.com/console-rs/indicatif)


### Notes

[chrono start of mont/week](https://users.rust-lang.org/t/how-to-find-first-day-of-current-month-and-year-using-chrono/51915)
[day in month trait](https://github.com/chronotope/chrono/issues/29#issuecomment-1510506317)