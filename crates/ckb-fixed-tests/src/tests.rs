use crate::{
    from_num, from_str, i64f64_add, i64f64_ln, i64f64_pow, initialize_wasmer, new, to_le_bytes,
};

#[test]
fn test_basic() {
    let (mut store, instance) = initialize_wasmer();

    let value = from_num(&mut store, &instance, 42).unwrap();
    let value2 = from_num(&mut store, &instance, 1).unwrap();
    let result = i64f64_add(&mut store, &instance, value, value2);
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
