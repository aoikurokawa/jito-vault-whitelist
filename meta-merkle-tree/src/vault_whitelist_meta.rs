use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

use crate::pubkey_string_conversion;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct VaultWhitelistMeta {
    /// User's pubkey
    #[serde(with = "pubkey_string_conversion")]
    pub user: Pubkey,
}
