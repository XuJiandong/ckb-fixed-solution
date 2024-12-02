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

// bindings to fixed crate
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
        let inner = a + b;
        I64F64 { inner }
    }
    pub fn sub(&self, b: &I64F64) -> I64F64 {
        let a = self.inner;
        let b = b.inner;
        let inner = a - b;
        I64F64 { inner }
    }
    pub fn mul(&self, b: &I64F64) -> I64F64 {
        let a = self.inner;
        let b = b.inner;
        let inner = a * b;
        I64F64 { inner }
    }
    pub fn div(&self, b: &I64F64) -> I64F64 {
        let a = self.inner;
        let b = b.inner;
        let inner = a / b;
        I64F64 { inner }
    }

    pub fn floor(&self) -> I64F64 {
        let a = self.inner;
        let inner = a.floor();
        I64F64 { inner }
    }

    pub fn ceil(&self) -> I64F64 {
        let a = self.inner;
        let inner = a.ceil();
        I64F64 { inner }
    }

    pub fn round(&self) -> I64F64 {
        let a = self.inner;
        let inner = a.round();
        I64F64 { inner }
    }
}

// bindings to transcendental module
#[wasm_bindgen]
impl I64F64 {
    pub fn sin(&self) -> I64F64 {
        let a = self.inner;
        let inner = transcendental::sin(a);
        I64F64 { inner }
    }
    pub fn cos(&self) -> I64F64 {
        let a = self.inner;
        let inner = transcendental::cos(a);
        I64F64 { inner }
    }
    pub fn exp(&self) -> Result<Self, JsValue> {
        let a = self.inner;
        let inner =
            transcendental::exp(a).map_err(|_| JsValue::from_str("exp calculation failed"))?;
        Ok(Self { inner })
    }
    pub fn ln(&self) -> Result<Self, JsValue> {
        let a = self.inner;
        let inner =
            transcendental::ln(a).map_err(|_| JsValue::from_str("ln calculation failed"))?;
        Ok(Self { inner })
    }
    pub fn pow(&self, b: &I64F64) -> Result<Self, JsValue> {
        let a = self.inner;
        let b = b.inner;
        let inner =
            transcendental::pow(a, b).map_err(|_| JsValue::from_str("pow calculation failed"))?;
        Ok(Self { inner })
    }
    pub fn tan(&self) -> I64F64 {
        let a = self.inner;
        let inner = transcendental::tan(a);
        I64F64 { inner }
    }
    pub fn sqrt(&self) -> Result<Self, JsValue> {
        let a = self.inner;
        let inner =
            transcendental::sqrt(a).map_err(|_| JsValue::from_str("sqrt calculation failed"))?;
        Ok(Self { inner })
    }
    pub fn log2(&self) -> Result<Self, JsValue> {
        let a = self.inner;
        let inner =
            transcendental::log2(a).map_err(|_| JsValue::from_str("log2 calculation failed"))?;
        Ok(Self { inner })
    }
    pub fn powi(&self, n: i32) -> Result<Self, JsValue> {
        let a = self.inner;
        let inner =
            transcendental::powi(a, n).map_err(|_| JsValue::from_str("powi calculation failed"))?;
        Ok(Self { inner })
    }
}
