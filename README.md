# ckb-fixed-solution






## Bug found by Fuzzing
We've implemented [fuzzing](./crates/ckb-fixed-tests/fuzz) for the ckb-fixed
crates. The fuzzer executes WASM bytecode using
[wasmer](https://github.com/wasmerio/wasmer) and compares the results with
native Rust code execution.

Findings from fuzzing:

- [panic in log2](https://github.com/XuJiandong/ckb-fixed-solution/pull/1/commits/1e50dc4df7b521c0e8b35839d3cec0bd87ce8e34#diff-594adab37e284643cf296678c3be41f68995df76e36e888cbe6a742cb39ff19eL215)
- [overflow in exp]()
- [very low performance on sin]()
- [negate with overflow in exp]()

Basically, these are not bugs in the WASM implementation but rather in the native implementation.
