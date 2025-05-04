use serde::{Deserialize, Serialize};
use solana_program::{
    hash::{Hash, Hasher},
    pubkey::Pubkey,
};

use crate::{pubkey_string_conversion, vault_whitelist_meta::VaultWhitelistMeta};

#[derive(Clone, Eq, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct VaultWhitelistMetaTreeNode {
    /// The depositor
    #[serde(with = "pubkey_string_conversion")]
    pub depositor: Pubkey,

    /// The proof associated with this TreeNode
    pub proof: Option<Vec<[u8; 32]>>,
}

impl VaultWhitelistMetaTreeNode {
    pub(crate) fn to_vec(vault_whitelist_metas: &[VaultWhitelistMeta]) -> Vec<Self> {
        let mut tree_nodes = Vec::new();
        for vault_whitelist_meta in vault_whitelist_metas {
            let tree_node = Self {
                depositor: vault_whitelist_meta.user,
                proof: None,
            };

            tree_nodes.push(tree_node);
        }

        tree_nodes
    }

    pub(crate) fn hash(&self) -> Hash {
        let mut hasher = Hasher::default();
        hasher.hash(self.depositor.as_ref());
        hasher.result()
    }
}
