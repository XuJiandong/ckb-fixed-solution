use crate::{
    from_num, from_str, i64f64_add, i64f64_exp, i64f64_ln, i64f64_pow, i64f64_sin,
    initialize_wasmer, new, to_le_bytes,
};

#[test]
fn test_basic() {
    let (mut store, instance) = initialize_wasmer();

    let value = from_num(&mut store, &instance, 42).unwrap();
    let value2 = from_num(&mut store, &instance, 1).unwrap();
    let result = i64f64_add(&mut store, &instance, value, value2).unwrap();
    let rust_value = ckb_fixed::I64F64::from_num(43).unwrap();

    assert_eq!(
        rust_value.to_le_bytes(),
        to_le_bytes(&mut store, &instance, result)
    );
    let new_value = new(&mut store, &instance, &rust_value.to_le_bytes()).unwrap();
    assert_eq!(
        rust_value.to_le_bytes(),
        to_le_bytes(&mut store, &instance, new_value)
    );
}

#[test]
fn test_transcendental() {
    let (mut store, instance) = initialize_wasmer();
    let a = 42;
    let b = 3;

    let value = from_num(&mut store, &instance, a).unwrap();
    let value2 = from_num(&mut store, &instance, b).unwrap();
    let result = i64f64_pow(&mut store, &instance, value, value2).unwrap();
    let rust_value = ckb_fixed::I64F64::pow(
        &ckb_fixed::I64F64::from_num(a).unwrap(),
        &ckb_fixed::I64F64::from_num(b).unwrap(),
    )
    .unwrap();

    assert_eq!(
        rust_value.to_le_bytes(),
        to_le_bytes(&mut store, &instance, result)
    );
    let new_value = new(&mut store, &instance, &rust_value.to_le_bytes()).unwrap();
    assert_eq!(
        rust_value.to_le_bytes(),
        to_le_bytes(&mut store, &instance, new_value)
    );
}

#[test]
fn test_from_str() {
    let (mut store, instance) = initialize_wasmer();
    let const_value = "123.45";
    let value = from_str(&mut store, &instance, const_value).unwrap();
    let rust_value = ckb_fixed::I64F64::from_str(const_value).unwrap();
    assert_eq!(
        rust_value.to_le_bytes(),
        to_le_bytes(&mut store, &instance, value)
    );
}

#[test]
fn test_return_error() {
    let (mut store, instance) = initialize_wasmer();
    let a = -42;

    let value = from_num(&mut store, &instance, a).unwrap();
    let result = i64f64_ln(&mut store, &instance, value);
    assert!(result.is_err());
}

#[test]
fn test_native_return_error() {
    let a = crate::ckb_fixed::I64F64::from_num(-42).unwrap();
    let result = a.ln();
    assert!(result.is_err());
}

// test cases from fuzzing
#[test]
fn test_fuzzing() {
    let bytes = [
        2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let (mut store, instance) = initialize_wasmer();
    let wasm_value = new(&mut store, &instance, &bytes).unwrap();
    let result = i64f64_ln(&mut store, &instance, wasm_value);
    assert!(result.is_err());

    let rust_value = ckb_fixed::I64F64::new(&bytes).unwrap();
    let rust_result = rust_value.ln();
    assert!(rust_result.is_err());
}

#[test]
fn test_fuzzing_2() {
    let bytes = [
        0x0a, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80,
    ];
    let (mut store, instance) = initialize_wasmer();
    let wasm_value = new(&mut store, &instance, &bytes).unwrap();
    let result = i64f64_exp(&mut store, &instance, wasm_value);
    assert!(result.is_err());

    let rust_value = ckb_fixed::I64F64::new(&bytes).unwrap();
    let rust_result = rust_value.exp();
    assert!(rust_result.is_err());
}

#[test]
fn test_fuzzing_3() {
    let bytes = [
        0x0a, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80,
    ];
    let rust_value = ckb_fixed::I64F64::new(&bytes).unwrap();
    let rust_result = rust_value.sin();
    assert!(rust_result.is_ok());

    let (mut store, instance) = initialize_wasmer();
    let wasm_value = new(&mut store, &instance, &bytes).unwrap();
    let result = i64f64_sin(&mut store, &instance, wasm_value);
    assert!(result.is_ok());
}

#[test]
fn test_fuzzing_4() {
    let bytes = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80,
    ];
    let rust_value = ckb_fixed::I64F64::new(&bytes).unwrap();
    let rust_result = rust_value.exp();
    assert!(rust_result.is_err());

    let (mut store, instance) = initialize_wasmer();
    let wasm_value = new(&mut store, &instance, &bytes).unwrap();
    let result = i64f64_exp(&mut store, &instance, wasm_value);
    assert!(result.is_err());
}
