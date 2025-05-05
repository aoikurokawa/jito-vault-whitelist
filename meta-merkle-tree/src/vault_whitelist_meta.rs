use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

use crate::pubkey_string_conversion;

/// Represents a user to be whitelisted in a vault system.
///
/// This structure holds the information about a user that should be
/// included in the whitelist. It contains the user's public key, which
/// is used to build the Merkle tree for whitelist verification.
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct VaultWhitelistMeta {
    /// User's pubkey
    #[serde(with = "pubkey_string_conversion")]
    pub user: Pubkey,
}

impl VaultWhitelistMeta {
    /// Initiaze new vault whitelist meta.
    pub const fn new(user: Pubkey) -> Self {
        Self { user }
    }
}
