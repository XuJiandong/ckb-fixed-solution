#![no_main]

use ckb_fixed_tests::ckb_fixed::I64F64;
use ckb_fixed_tests::{
    i64f64_exp, i64f64_ln, i64f64_log2, i64f64_pow, i64f64_sin, i64f64_sqrt, initialize_wasmer,
    new, to_le_bytes, Instance, Store,
};
use libfuzzer_sys::fuzz_target;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static WASMER: Lazy<Mutex<(Store, Instance)>> = Lazy::new(|| Mutex::new(initialize_wasmer()));
static mut COUNT: u32 = 0;

fn get_count() -> u32 {
    unsafe { COUNT }
}

fn tick_count() {
    unsafe {
        COUNT += 1;
    }
}
fn reset_count() {
    unsafe {
        COUNT = 0;
    }
}

fuzz_target!(|data: &[u8]| {
    tick_count();
    // wasmer became unstable when called too many times.
    if get_count() > 100000 {
        let (new_store, new_instance) = initialize_wasmer();
        let mut wasmer_guard = WASMER.lock().unwrap();
        *wasmer_guard = (new_store, new_instance);
        reset_count();
    }
    let (store, instance) = &mut *WASMER.lock().unwrap();

    if data.len() == 16 {
        // ln
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

        // log2
        let wasm_result = i64f64_log2(store, instance, wasm_value);
        let native_result = native_value.log2();
        if wasm_result.is_err() {
            assert!(native_result.is_err());
        } else {
            let wasm_bytes = to_le_bytes(store, instance, wasm_result.unwrap());
            let native_bytes = native_result.unwrap().to_le_bytes();
            assert_eq!(wasm_bytes, native_bytes);
        }
        // exp
        let wasm_result = i64f64_exp(store, instance, wasm_value);
        let native_result = native_value.exp();
        if wasm_result.is_err() {
            assert!(native_result.is_err());
        } else {
            let wasm_bytes = to_le_bytes(store, instance, wasm_result.unwrap());
            let native_bytes = native_result.unwrap().to_le_bytes();
            assert_eq!(wasm_bytes, native_bytes);
        }
        // sqrt
        let wasm_result = i64f64_sqrt(store, instance, wasm_value);
        let native_result = native_value.sqrt();
        if wasm_result.is_err() {
            assert!(native_result.is_err());
        } else {
            let wasm_bytes = to_le_bytes(store, instance, wasm_result.unwrap());
            let native_bytes = native_result.unwrap().to_le_bytes();
            assert_eq!(wasm_bytes, native_bytes);
        }
        // sin
        let wasm_result = i64f64_sin(store, instance, wasm_value);
        let native_result = native_value.sin();
        if wasm_result.is_err() {
            assert!(native_result.is_err());
        } else {
            let wasm_bytes = to_le_bytes(store, instance, wasm_result.unwrap());
            let native_bytes = native_result.unwrap().to_le_bytes();
            assert_eq!(wasm_bytes, native_bytes);
        }
    }
    if data.len() == 32 {
        // pow
        let wasm_value1 = new(store, instance, &data[..16]).unwrap();
        let wasm_value2 = new(store, instance, &data[16..]).unwrap();
        let wasm_result = i64f64_pow(store, instance, wasm_value1, wasm_value2);
        let native_value1 = I64F64::new(&data[..16]).unwrap();
        let native_value2 = I64F64::new(&data[16..]).unwrap();
        let native_result = native_value1.pow(&native_value2);
        if wasm_result.is_err() {
            assert!(native_result.is_err());
        } else {
            let wasm_bytes = to_le_bytes(store, instance, wasm_result.unwrap());
            let native_bytes = native_result.unwrap().to_le_bytes();
            assert_eq!(wasm_bytes, native_bytes);
        }
    }
});
