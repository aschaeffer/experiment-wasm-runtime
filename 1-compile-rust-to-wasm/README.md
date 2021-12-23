## Compile RUST to WASM

## 1. Install WASI-enabled Rust toolchain

```shell
$ rustup target add wasm32-wasi
```

## 2. Install wasmtime

https://wasmtime.dev/

```shell
curl https://wasmtime.dev/install.sh -sSf | bash
```

## 3. Build project 

```shell
$ cargo build --target wasm32-wasi
```

## 4. Result

|        | Language | File                                                    |
|--------|----------|---------------------------------------------------------|
| Source | Rust     | `src/main.js`                                           |
| Target | WASM     | `target/wasm32-wasi/debug/experiment-wasm-runtime.wasm` |

```shell
$ file target/wasm32-wasi/debug/rust-to-wasm.wasm
target/wasm32-wasi/debug/rust-to-wasm.wasm: WebAssembly (wasm) binary module version 0x1 (MVP)
```

## 5. Execute

```shell
$ echo "test" > sourcefile.txt
$ wasmtime --dir=. --dir=/tmp target/wasm32-wasi/debug/rust-to-wasm.wasm ./sourcefile.txt /tmp/targetfile.txt
$ cat /tmp/targetfile.txt 
test
```

## 6. Difficulty to compile from Rust to WASM

Low 😀

## 7. Binary Size

```toml
[profile.release]
# opt-level = "s" --> execution speed
# opt-level = "z" --> binary size
opt-level = "z"
lto = true
strip = "symbols"
```

|                   | Debug  | Release   |
|-------------------|--------|-----------|
| rust-to-wasm.wasm | 2.1M   | 191 Bytes |
