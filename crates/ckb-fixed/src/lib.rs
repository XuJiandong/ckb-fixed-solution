#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod transcendental;
use alloc::vec::Vec;
pub use fixed::types;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct I64F64 {
    inner: types::I64F64,
}

#[wasm_bindgen]
impl I64F64 {
    pub fn new(inner: &[u8]) -> Result<Self, JsValue> {
        let inner = types::I64F64::from_le_bytes(
            inner
                .try_into()
                .map_err(|_| JsValue::from_str("Invalid length"))?,
        );
        Ok(Self { inner })
    }
    pub fn from_str(s: &str) -> Result<I64F64, JsValue> {
        let inner = types::I64F64::from_str(s).map_err(|_| JsValue::from_str("Invalid number"))?;
        Ok(I64F64 { inner })
    }
    pub fn from_num(n: i64) -> Result<I64F64, JsValue> {
        let inner = types::I64F64::from_num(n);
        Ok(I64F64 { inner })
    }
    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.inner.to_le_bytes().to_vec()
    }
    pub fn add(&self, b: &I64F64) -> I64F64 {
        let a = self.inner;
        let b = b.inner;
        let c = a + b;
        I64F64 { inner: c }
    }
    pub fn sub(&self, b: &I64F64) -> I64F64 {
        let a = self.inner;
        let b = b.inner;
        let c = a - b;
        I64F64 { inner: c }
    }
}
