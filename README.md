desktop fast build / run:

```
cargo run --features bevy/dynamic_linking
```

or web

```

rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner

cargo run --target wasm32-unknown-unknown
```
