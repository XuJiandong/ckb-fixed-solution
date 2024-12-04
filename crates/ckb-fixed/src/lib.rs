#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::should_implement_trait)]
//! fixed point support for CKB on-chain script.
//!
//! This crate is based on [fixed](https://crates.io/crates/fixed).
//!
//! # Overview
//!
//! This crate provides fixed-point arithmetic types optimized for CKB smart contracts.
//! Fixed-point numbers are useful when decimal precision is needed but floating-point
//! operations are not available or desired. The default type is I64F64, which is a
//! 64-bit integer with 64 fractional bits.
//!
//! # Features
//!
//! - Fixed-point arithmetic with configurable precision
//! - Basic mathematical operations (+, -, *, /, %)
//! - Implements common traits like `Add`, `Sub`, `Mul`, `Div`
//! - Transcendental functions like `exp`, `ln`, `pow`, `log2`, `sin`, etc.
//! - No floating-point dependencies
//!
//! # Example
//!
//! ```rust,ignore
//! use ckb_fixed::types::I64F64;
//!
//! let a = I64F64::from_num(5).unwrap();
//! let result = a.ln().unwrap();
//! ```
extern crate alloc;

pub mod transcendental;
use alloc::vec::Vec;
pub use fixed::types;
#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum FixedError {
    InvalidLength,
    InvalidNumber,
    Calculation(&'static str),
}

#[cfg(feature = "wasm-bindgen")]
impl From<FixedError> for JsValue {
    fn from(error: FixedError) -> JsValue {
        match error {
            FixedError::InvalidLength => JsValue::from_str("Invalid length"),
            FixedError::InvalidNumber => JsValue::from_str("Invalid number"),
            FixedError::Calculation(msg) => JsValue::from_str(msg),
        }
    }
}

/// The fixed-point default number type, 64-bit integer with 64 fractional bits
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq)]
pub struct I64F64 {
    inner: types::I64F64,
}

impl From<I64F64> for types::I64F64 {
    fn from(v: I64F64) -> Self {
        v.inner
    }
}

impl From<types::I64F64> for I64F64 {
    fn from(v: types::I64F64) -> Self {
        I64F64 { inner: v }
    }
}

// bindings to fixed crate
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
impl I64F64 {
    #[cfg(feature = "wasm-bindgen")]
    #[wasm_bindgen(js_name = toJSON)]
    /// Convert the `I64F64` to a JSON string.
    pub fn to_json(&self) -> String {
        self.inner.to_string()
    }
    #[cfg(feature = "wasm-bindgen")]
    #[wasm_bindgen(js_name = toString)]
    /// Convert the `I64F64` to a string.
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }
    /// Create a new `I64F64` from a byte slice.
    pub fn new(inner: &[u8]) -> Result<Self, FixedError> {
        let inner =
            types::I64F64::from_le_bytes(inner.try_into().map_err(|_| FixedError::InvalidLength)?);
        Ok(Self { inner })
    }
    /// Create a new `I64F64` from a string.
    pub fn from_str(s: &str) -> Result<I64F64, FixedError> {
        let inner = types::I64F64::from_str(s).map_err(|_| FixedError::InvalidNumber)?;
        Ok(I64F64 { inner })
    }
    /// Create a new `I64F64` from an integer.
    pub fn from_num(n: i64) -> Result<I64F64, FixedError> {
        let inner = types::I64F64::from_num(n);
        Ok(I64F64 { inner })
    }
    /// Convert the `I64F64` to a byte slice.
    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.inner.to_le_bytes().to_vec()
    }
    /// Add two `I64F64` numbers.
    pub fn add(&self, b: &I64F64) -> I64F64 {
        let a = self.inner;
        let b = b.inner;
        let inner = a + b;
        I64F64 { inner }
    }
    /// Subtract two `I64F64` numbers.
    pub fn sub(&self, b: &I64F64) -> I64F64 {
        let a = self.inner;
        let b = b.inner;
        let inner = a - b;
        I64F64 { inner }
    }
    /// Multiply two `I64F64` numbers.
    pub fn mul(&self, b: &I64F64) -> I64F64 {
        let a = self.inner;
        let b = b.inner;
        let inner = a * b;
        I64F64 { inner }
    }
    /// Divide `I64F64` numbers.
    pub fn div(&self, b: &I64F64) -> I64F64 {
        let a = self.inner;
        let b = b.inner;
        let inner = a / b;
        I64F64 { inner }
    }
    /// Rounds to the next integer towards −∞.
    pub fn floor(&self) -> I64F64 {
        let a = self.inner;
        let inner = a.floor();
        I64F64 { inner }
    }
    /// Rounds to the next integer towards +∞.
    pub fn ceil(&self) -> I64F64 {
        let a = self.inner;
        let inner = a.ceil();
        I64F64 { inner }
    }
    /// Rounds to the nearest integer, with ties rounded away from zero.
    pub fn round(&self) -> I64F64 {
        let a = self.inner;
        let inner = a.round();
        I64F64 { inner }
    }
    /// Check if two `I64F64` numbers are equal.
    pub fn eq(&self, b: &I64F64) -> bool {
        self.inner == b.inner
    }
    /// Check if `I64F64` number is less than another.
    pub fn lt(&self, b: &I64F64) -> bool {
        self.inner < b.inner
    }
    /// Check if `I64F64` number is greater than another.
    pub fn gt(&self, b: &I64F64) -> bool {
        self.inner > b.inner
    }
    /// Check if `I64F64` number is less than or equal to another.
    pub fn le(&self, b: &I64F64) -> bool {
        self.inner <= b.inner
    }
    /// Check if `I64F64` number is greater than or equal to another.
    pub fn ge(&self, b: &I64F64) -> bool {
        self.inner >= b.inner
    }
}

// bindings to transcendental module
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
impl I64F64 {
    /// Calculate the exponential of `I64F64` number.
    pub fn exp(&self) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner = transcendental::exp(a)
            .map_err(|_| FixedError::Calculation("exp calculation failed"))?;
        Ok(Self { inner })
    }
    /// Calculate the natural logarithm of `I64F64` number.
    pub fn ln(&self) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner =
            transcendental::ln(a).map_err(|_| FixedError::Calculation("ln calculation failed"))?;
        Ok(Self { inner })
    }
    /// Calculate the power of `I64F64` number.
    pub fn pow(&self, b: &I64F64) -> Result<Self, FixedError> {
        let a = self.inner;
        let b = b.inner;
        let inner = transcendental::pow(a, b)
            .map_err(|_| FixedError::Calculation("pow calculation failed"))?;
        Ok(Self { inner })
    }
    /// Calculate the square root of `I64F64` number.
    pub fn sqrt(&self) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner = transcendental::sqrt(a)
            .map_err(|_| FixedError::Calculation("sqrt calculation failed"))?;
        Ok(Self { inner })
    }
    /// Calculate the base-2 logarithm of `I64F64` number.
    pub fn log2(&self) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner = transcendental::log2(a)
            .map_err(|_| FixedError::Calculation("log2 calculation failed"))?;
        Ok(Self { inner })
    }
    /// Calculate the power of `I64F64` number with an integer exponent.
    pub fn powi(&self, n: i32) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner = transcendental::powi(a, n)
            .map_err(|_| FixedError::Calculation("powi calculation failed"))?;
        Ok(Self { inner })
    }
    /// Calculate the sine of `I64F64` number.
    pub fn sin(&self) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner = transcendental::sin(a)
            .map_err(|_| FixedError::Calculation("sin calculation failed"))?;
        Ok(Self { inner })
    }
    /// Calculate the cosine of `I64F64` number.
    pub fn cos(&self) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner = transcendental::cos(a)
            .map_err(|_| FixedError::Calculation("cos calculation failed"))?;
        Ok(Self { inner })
    }
    /// Calculate the tangent of `I64F64` number.
    pub fn tan(&self) -> Result<Self, FixedError> {
        let a = self.inner;
        let inner = transcendental::tan(a)
            .map_err(|_| FixedError::Calculation("tan calculation failed"))?;
        Ok(Self { inner })
    }
}
