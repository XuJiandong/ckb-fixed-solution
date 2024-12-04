# ckb-fixed-solution






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
