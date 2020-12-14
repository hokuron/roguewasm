# roguewasm

[Programming WebAssembly with Rust](https://pragprog.com/titles/khrust/programming-webassembly-with-rust/)

## How to play

    cargo build --release --target wasm32-unknown-unknown 
    wasm-bindgen target/wasm32-unknown-unknown/release/roguewasm.wasm --out-dir .
    
    npm install
    npm run serve
