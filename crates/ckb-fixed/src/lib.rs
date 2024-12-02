#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::should_implement_trait)]

extern crate alloc;

pub mod transcendental;
use alloc::vec::Vec;
pub use fixed::types;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum FixedError {
    InvalidLength,
    InvalidNumber,
    Calculation(&'static str),
}

impl From<FixedError> for JsValue {
    fn from(error: FixedError) -> JsValue {
        match error {
            FixedError::InvalidLength => JsValue::from_str("Invalid length"),
            FixedError::InvalidNumber => JsValue::from_str("Invalid number"),
            FixedError::Calculation(msg) => JsValue::from_str(msg),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct I64F64 {
    inner: types::I64F64,
}

// bindings to fixed crate
#[wasm_bindgen]
impl I64F64 {
    pub fn new(inner: &[u8]) -> Result<Self, FixedError> {
        let inner =
            types::I64F64::from_le_bytes(inner.try_into().map_err(|_| FixedError::InvalidLength)?);
        Ok(Self { inner })
    }
    pub fn from_str(s: &str) -> Result<I64F64, FixedError> {
        let inner = types::I64F64::from_str(s).map_err(|_| FixedError::InvalidNumber)?;
        Ok(I64F64 { inner })
    }
    pub fn from_num(n: i64) -> Result<I64F64, FixedError> {
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
    pub fn exp(&self) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner = transcendental::exp(a)
            .map_err(|_| FixedError::Calculation("exp calculation failed"))?;
        Ok(Self { inner })
    }
    pub fn ln(&self) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner =
            transcendental::ln(a).map_err(|_| FixedError::Calculation("ln calculation failed"))?;
        Ok(Self { inner })
    }
    pub fn pow(&self, b: &I64F64) -> Result<Self, FixedError> {
        let a = self.inner;
        let b = b.inner;
        let inner = transcendental::pow(a, b)
            .map_err(|_| FixedError::Calculation("pow calculation failed"))?;
        Ok(Self { inner })
    }
    pub fn tan(&self) -> I64F64 {
        let a = self.inner;
        let inner = transcendental::tan(a);
        I64F64 { inner }
    }
    pub fn sqrt(&self) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner = transcendental::sqrt(a)
            .map_err(|_| FixedError::Calculation("sqrt calculation failed"))?;
        Ok(Self { inner })
    }
    pub fn log2(&self) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner = transcendental::log2(a)
            .map_err(|_| FixedError::Calculation("log2 calculation failed"))?;
        Ok(Self { inner })
    }
    pub fn powi(&self, n: i32) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner = transcendental::powi(a, n)
            .map_err(|_| FixedError::Calculation("powi calculation failed"))?;
        Ok(Self { inner })
    }
}
