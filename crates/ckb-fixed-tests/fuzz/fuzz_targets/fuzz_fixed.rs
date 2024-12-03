#![no_main]

use ckb_fixed_tests::ckb_fixed::I64F64;
use ckb_fixed_tests::{i64f64_ln, initialize_wasmer, new, to_le_bytes};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let (mut store, instance) = initialize_wasmer();
    if data.len() == 16 {
        let wasm_value = new(&mut store, &instance, data).unwrap();
        let native_value = I64F64::new(data).unwrap();

        let wasm_result = i64f64_ln(&mut store, &instance, wasm_value);
        let native_result = native_value.ln();
        if wasm_result.is_err() {
            assert!(native_result.is_err());
        } else {
            let wasm_bytes = to_le_bytes(&mut store, &instance, wasm_result.unwrap());
            let native_bytes = native_result.unwrap().to_le_bytes();
            assert_eq!(wasm_bytes, native_bytes);
        }
    }
});
