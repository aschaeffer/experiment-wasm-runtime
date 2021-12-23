# Embed the WASM runtime

## How it works:

1. `2-embed-wasm-runtime` is a rust program which hosts a wasm runtime
2. `3-rust-wasm-library` produces a wasm library (in contrast to `1-compile-rust-to-wasm`)
3. `3-rust-wasm-library` exports a function named `answer` (no-mangle ==> untouched function name)
4. `2-embed-wasm-runtime` loads the compiled `wasm`
5. `2-embed-wasm-runtime` calls the function named `answer` and gets a result
6. `2-embed-wasm-runtime` prints the answer
