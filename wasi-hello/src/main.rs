use std::io::prelude::*;
use std::fs;

/*
BUILD:
rustup target add wasm32-wasi
cargo build --target wasm32-wasi

RUN:
wasmtime --env WASMTIME_BACKTRACE_DETAILS=1 --dir .::. target/wasm32-wasi/debug/wasi-hello.wasm
*/

fn main() {
    println!("Hello, WASI!!!");

    let mut file = fs::File::create("/files/test.txt").unwrap();
    write!(file, "Hello, WASI!!!").unwrap();
}
