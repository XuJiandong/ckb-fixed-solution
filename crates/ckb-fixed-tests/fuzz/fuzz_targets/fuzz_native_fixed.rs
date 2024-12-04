#![no_main]

use ckb_fixed_tests::ckb_fixed::I64F64;
use libfuzzer_sys::fuzz_target;
fuzz_target!(|data: &[u8]| {
    if data.len() == 16 {
        let native_value = I64F64::new(data).unwrap();
        // ln
        let _ = native_value.ln();
        // log2
        let _ = native_value.log2();
        // exp
        let _ = native_value.exp();
        // sqrt
        let _ = native_value.sqrt();
        // sin
        let _ = native_value.sin();
    }
    if data.len() == 32 {
        // pow
        let native_value1 = I64F64::new(&data[..16]).unwrap();
        let native_value2 = I64F64::new(&data[16..]).unwrap();
        let _ = native_value1.pow(&native_value2);
    }
});
