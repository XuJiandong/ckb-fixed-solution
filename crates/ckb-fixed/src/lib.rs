#![cfg_attr(not(feature = "std"), no_std)]

pub mod transcendental;
use fixed::types;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct I64F64 {
    inner: [u8; 16],
}

#[wasm_bindgen]
impl I64F64 {
    pub fn new(inner: &[u8]) -> Result<Self, JsValue> {
        let inner: [u8; 16] = inner
            .try_into()
            .map_err(|_| JsValue::from_str("Invalid length"))?;
        Ok(Self { inner })
    }
    pub fn from_str(s: &str) -> Result<I64F64, JsValue> {
        let inner = types::I64F64::from_str(s).map_err(|_| JsValue::from_str("Invalid number"))?;
        Ok(I64F64 {
            inner: inner.to_le_bytes(),
        })
    }
    pub fn from_num(n: u64) -> Result<I64F64, JsValue> {
        let inner = types::I64F64::from_num(n);
        Ok(I64F64 {
            inner: inner.to_le_bytes(),
        })
    }
    pub fn add(&self, b: &I64F64) -> I64F64 {
        let a = types::I64F64::from_le_bytes(self.inner);
        let b = types::I64F64::from_le_bytes(b.inner);
        let c = a + b;
        I64F64 {
            inner: c.to_le_bytes(),
        }
    }
    pub fn sub(&self, b: &I64F64) -> I64F64 {
        let a = types::I64F64::from_le_bytes(self.inner);
        let b = types::I64F64::from_le_bytes(b.inner);
        let c = a - b;
        I64F64 {
            inner: c.to_le_bytes(),
        }
    }
}
