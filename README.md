# ckb-fixed-solution
When developing on-chain scripts for CKB, numeric calculations are often required.
Some developers prefer using [fixed-point arithmetic](https://en.wikipedia.org/wiki/Fixed-point_arithmetic)
due to its simplicity. To ensure consistent calculations between on-chain scripts
and off-chain applications, we provide a solution based on WASM and Rust.

This solution consists of two parts:
1. A Rust library ([ckb-fixed](./crates/ckb-fixed)) for on-chain scripts. It is based on [fixed](https://crates.io/crates/fixed).
2. WASM bindings of the same Rust library for off-chain applications, generated using `wasm-pack`

This approach ensures identical numerical behavior across both environments.

## How to Integrate

### Prerequisites
Install required tools:
```bash
# Install wasm-pack for WASM bindings generation
cargo install wasm-pack --version 0.13.1

# Add WebAssembly target to Rust
rustup target add wasm32-unknown-unknown
```

### For Off-chain Applications (JavaScript/WASM)
Generate WASM bindings:
```bash
cd crates/ckb-fixed
make wasm-pack
```
The generated WASM bindings will be available in the `pkg` directory. For usage instructions, refer to the [wasm-pack documentation](https://rustwasm.github.io/docs/wasm-pack/).

### For On-chain Scripts (Rust)
Add the dependency to your `Cargo.toml`:
```toml
[dependencies]
ckb-fixed = "1.0.0"
```

For implementation examples, see our [on-chain script example](./contracts/fixed-script-example).

## Bugs found by Fuzzing
We've implemented [fuzzing](./crates/ckb-fixed-tests/fuzz) for the ckb-fixed
crates. The fuzzer executes WASM bytecode using
[wasmer](https://github.com/wasmerio/wasmer) and compares the results with
native Rust code execution.

Findings from fuzzing:

- [panic in log2](https://github.com/XuJiandong/ckb-fixed-solution/pull/1/commits/1e50dc4df7b521c0e8b35839d3cec0bd87ce8e34#diff-594adab37e284643cf296678c3be41f68995df76e36e888cbe6a742cb39ff19eL215)
- [overflow in exp](https://github.com/XuJiandong/ckb-fixed-solution/pull/4/commits/036745abf00aed45942b1afd88cebc6f8b64abc8#diff-594adab37e284643cf296678c3be41f68995df76e36e888cbe6a742cb39ff19eR263)
- [very low performance on sin](https://github.com/XuJiandong/ckb-fixed-solution/pull/4/commits/036745abf00aed45942b1afd88cebc6f8b64abc8#diff-594adab37e284643cf296678c3be41f68995df76e36e888cbe6a742cb39ff19eR404)
- [negate with overflow in exp](https://github.com/XuJiandong/ckb-fixed-solution/pull/4/commits/036745abf00aed45942b1afd88cebc6f8b64abc8#diff-594adab37e284643cf296678c3be41f68995df76e36e888cbe6a742cb39ff19eR259)

Basically, these are not bugs in the WASM implementation but rather in the native implementation.
