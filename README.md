# Rust runtime for OP_NET VM

* Rust contract is able to run under [op-vm](https://github.com/btc-vision/op-vm)
* The interface is defined by [btc-runtime](https://github.com/btc-vision/btc-runtime)
* Library is tested like: [opnet-unit-test](https://github.com/btc-vision/opnet-unit-test)
* Reference contract: [OP_20](https://github.com/btc-vision/OP_20)

## Compilation

```sh
rustup install nightly # ensure that you have last nightly version
cargo build --release -p example --target wasm32-unknown-unknown
wasm-opt -O2 -Oz --strip-debug --strip-dwarf --dce --disable-multimemory --disable-fp16 --disable-mutable-globals --disable-gc --disable-multivalue --disable-nontrapping-float-to-int --disable-threads --mvp-features --remove-unused-module-elements ./target/wasm32-unknown-unknown/release/example.wasm -o ./rust.wasm
```

## Testing

### With opnet-unit-test on wasm interpreter

```sh
cp ./rust.wasm ../opnet-unit-test/bytecode/rust.wasm
node build/tests/rust.js
```

### Without wasm32 runtime

```sh
cargo test --target x86_64-unknown-linux-gnu
```

## Size (outdated)

* Compiled code from [moto library](https://github.com/btc-vision/OP_20): 24KB (24689)
* Unoptimized code from current Rust library: 45KB (45584)
* Optimized code with wasm-opt: 33KB (33573)

## Structure

* `./src/` - Library providing the interface and helper functions for contract interaction
* `./example/` - Sample smart contract, similar to contract used in unit tests
