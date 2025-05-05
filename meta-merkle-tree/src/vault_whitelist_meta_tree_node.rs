use serde::{Deserialize, Serialize};
use solana_program::{
    hash::{Hash, Hasher},
    pubkey::Pubkey,
};

use crate::{pubkey_string_conversion, vault_whitelist_meta::VaultWhitelistMeta};

/// Represents a node in the Merkle tree specifically for vault whitelist verification.
///
/// This structure is used to build the Merkle tree for vault whitelist verification.
/// Each node corresponds to a whitelisted user and contains the proof needed
/// to verify the depositor's inclusion in the whitelist.
#[derive(Clone, Eq, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct VaultWhitelistMetaTreeNode {
    /// The user
    #[serde(with = "pubkey_string_conversion")]
    pub user: Pubkey,

    /// The proof associated with this TreeNode
    pub proof: Option<Vec<[u8; 32]>>,
}

impl VaultWhitelistMetaTreeNode {
    /// Converts a slice of [`VaultWhitelistMeta`] to a vector of [`VaultWhitelistMetaTreeNode`].
    ///
    /// This function transforms each [`VaultWhitelistMeta`] into a corresponding
    /// [`VaultWhitelistMetaTreeNode`], which is needed for building the Merkle tree.
    pub(crate) fn to_vec(vault_whitelist_metas: &[VaultWhitelistMeta]) -> Vec<Self> {
        let mut tree_nodes = Vec::new();
        for vault_whitelist_meta in vault_whitelist_metas {
            let tree_node = Self {
                user: vault_whitelist_meta.user,
                proof: None,
            };

            tree_nodes.push(tree_node);
        }

        tree_nodes
    }

    /// Computes the hash of this tree node.
    ///
    /// This function creates a hash of the node's data, which is used when building the Merkle
    /// tree. The hash is based on the user's public key.
    pub(crate) fn hash(&self) -> Hash {
        let mut hasher = Hasher::default();
        hasher.hash(self.user.as_ref());
        hasher.result()
    }
}
