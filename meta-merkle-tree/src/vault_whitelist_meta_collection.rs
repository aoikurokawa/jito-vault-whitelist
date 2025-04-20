use serde::{Deserialize, Serialize};

use crate::vault_whitelist_meta::VaultWhitelistMeta;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultWhitelistMetaCollection {
    /// List of [VaultWhitelistMeta].
    pub vault_whitelist_metas: Vec<VaultWhitelistMeta>,
    // base58 encoded tip-distribution program id.
    // #[serde(with = "pubkey_string_conversion")]
    // pub tip_distribution_program_id: Pubkey,

    // Base58 encoded bank hash this object was generated at.
    // pub bank_hash: String,

    // Epoch for which this object was generated for.
    // pub epoch: Epoch,

    // Slot at which this object was generated.
    // pub slot: Slot,
}
