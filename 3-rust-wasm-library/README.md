## 7. Binary Size

The important setting is this:

```toml
[profile.release]
# opt-level = "s" --> execution speed
# opt-level = "z" --> binary size
opt-level = "z"
lto = true
strip = "symbols"
```

|                        | Target                 | Debug | Release   |
|------------------------|------------------------|-------|-----------|
| rust_wasm_library.wasm | wasm32-unknown-unknown | 1.6M  | 107 Bytes |
| rust_wasm_library.wasm | wasm32-wasi            | 1.7M  | 145 Bytes |
