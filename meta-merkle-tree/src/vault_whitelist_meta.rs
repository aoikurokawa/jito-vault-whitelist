use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::pubkey_string_conversion;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct VaultWhitelistMeta {
    /// Depositor's pubkey
    #[serde(with = "pubkey_string_conversion")]
    pub depositor_pubkey: Pubkey,
}
