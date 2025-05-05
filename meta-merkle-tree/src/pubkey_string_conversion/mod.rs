//! Module for Solana pubkey serialization and deserialization with Serde.
//!
//! This module provides utility functions to convert Solana `Pubkey` objects to strings
//! and back when serializing/deserializing with Serde. It's primarily used with the
//! `#[serde(with = "pubkey_string_conversion")]` attribute on struct fields.
//!
//! # Example
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use solana_program::pubkey::Pubkey;
//! use jito_vault_whitelist_meta_merkle_tree::pubkey_string_conversion;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Account {
//!     #[serde(with = "pubkey_string_conversion")]
//!     owner: Pubkey,
//! }
//! ```

use std::str::FromStr;

use serde::{self, Deserialize, Deserializer, Serializer};
use solana_program::pubkey::Pubkey;

/// Serializes a Solana `Pubkey` as a string.
///
/// This function converts a Solana `Pubkey` to its base-58 string representation
/// for JSON serialization and other text-based formats.
pub fn serialize<S>(pubkey: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&pubkey.to_string())
}

/// Deserializes a string into a Solana `Pubkey`.
///
/// This function converts a base-58 encoded string back into a Solana `Pubkey`
/// during deserialization from JSON and other text-based formats.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Pubkey::from_str(&s).map_err(serde::de::Error::custom)
}
