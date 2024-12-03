#![no_main]

use ckb_fixed_tests::ckb_fixed::I64F64;
use ckb_fixed_tests::{i64f64_ln, initialize_wasmer, new, to_le_bytes, Instance, Store};
use libfuzzer_sys::fuzz_target;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static WASMER: Lazy<Mutex<(Store, Instance)>> = Lazy::new(|| Mutex::new(initialize_wasmer()));

fuzz_target!(|data: &[u8]| {
    let (store, instance) = &mut *WASMER.lock().unwrap();

    if data.len() == 16 {
        let wasm_value = new(store, instance, data).unwrap();
        let native_value = I64F64::new(data).unwrap();

        let wasm_result = i64f64_ln(store, instance, wasm_value);
        let native_result = native_value.ln();
        if wasm_result.is_err() {
            assert!(native_result.is_err());
        } else {
            let wasm_bytes = to_le_bytes(store, instance, wasm_result.unwrap());
            let native_bytes = native_result.unwrap().to_le_bytes();
            assert_eq!(wasm_bytes, native_bytes);
        }
    }
});
