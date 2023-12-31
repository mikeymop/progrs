# Progrs

`progrs` (prog-R-S), is a wasm binary that gives you a progress bar for the current day, week, year.

![example](https://github.com/mikeymop/progrs/blob/main/example/progrs.gif?raw=true)

**Bugs:**

 - indicatif doesn't render on wasm32
 - timezone is always utc on wasm32

### Setup

You can build a traditional binary using `cargo build` or a `wasm` binary:

```shell
curl https://get.wasmer.io -sSfL | sh # Install wasmer runtime

# build for wasix (POSIX) target/wasm32-wasmer-wasi/
cargo install cargo-wasix
cargo wasix build # or cargo wasix build --release

# or build for wasi
rustup target add wasm32-wasi
cargo build --target=wasm32-wasi
```

This will build a `wasm` binary in `target/wasm32-wasmer-wasi/debug/progrs.wasm`. You can run this binary with `wasmer run`.

### Thank you to

 - [chrono](https://github.com/chronotope/chrono)
 - [indicatif](https://github.com/console-rs/indicatif)


### Notes

[chrono start of mont/week](https://users.rust-lang.org/t/how-to-find-first-day-of-current-month-and-year-using-chrono/51915)  
[day in month trait](https://github.com/chronotope/chrono/issues/29#issuecomment-1510506317)  
[clap is_terminal wasm breaks](https://github.com/clap-rs/clap/issues/4510#issuecomment-1327956501)  
[indicatif doesn't render on wasm32](https://github.com/console-rs/indicatif/issues/513#issuecomment-1567483572)  
[day in year](https://www.epochconverter.com/days/2023)  
