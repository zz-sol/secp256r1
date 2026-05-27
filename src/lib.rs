//! secp256r1/P-256 ECDSA keys, signatures, signing, and verification.
//!
//! # Security
//!
//! This crate is experimental and has not been audited. Signing and
//! signing-key import currently use variable-time scalar multiplication for
//! secret-dependent values. Do not use this crate for production signing or in
//! environments where local timing/cache side channels are in scope.

#![forbid(unsafe_code)]

mod constants;
mod ecdsa;
mod error;

pub mod field;
pub mod group;
pub mod scalar;

pub use ecdsa::{DerSignature, EncodedPoint, Signature, SigningKey, VerifyingKey};
pub use error::{Error, Result};
