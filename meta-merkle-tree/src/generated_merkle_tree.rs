use serde::{Deserialize, Serialize};
use solana_program::hash::Hash;
use solana_program::pubkey::Pubkey;

use crate::merkle_tree::MerkleTree;
use crate::pubkey_string_conversion;
use crate::utils::get_proof;
use crate::vault_whitelist_meta::VaultWhitelistMeta;
use crate::vault_whitelist_meta_tree_node::VaultWhitelistMetaTreeNode;

#[derive(Clone, Eq, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct GeneratedMerkleTree {
    /// User account (wallet pubkey)
    #[serde(with = "pubkey_string_conversion")]
    pub user_account: Pubkey,

    /// Merkle root
    pub merkle_root: Hash,

    /// Tree nodes
    pub tree_nodes: Vec<VaultWhitelistMetaTreeNode>,

    /// Maximum number of nodes
    pub max_num_nodes: u64,
}

impl GeneratedMerkleTree {
    pub fn new(user_account: &Pubkey, vault_whitelist_metas: &[VaultWhitelistMeta]) -> Self {
        let mut tree_nodes = VaultWhitelistMetaTreeNode::to_vec(vault_whitelist_metas);

        let hashed_nodes: Vec<[u8; 32]> = tree_nodes.iter().map(|n| n.hash().to_bytes()).collect();

        let merkle_tree = MerkleTree::new(&hashed_nodes[..], true);
        let max_num_nodes = tree_nodes.len() as u64;

        for (i, tree_node) in tree_nodes.iter_mut().enumerate() {
            tree_node.proof = Some(get_proof(&merkle_tree, i));
        }

        Self {
            max_num_nodes,
            user_account: *user_account,
            merkle_root: *merkle_tree.get_root().unwrap(),
            tree_nodes,
        }
    }

    pub fn get_proof(
        vault_whitelist_metas: &[VaultWhitelistMeta],
        depositor: &Pubkey,
    ) -> Vec<[u8; 32]> {
        let mut proof = Vec::new();

        let tree_nodes = VaultWhitelistMetaTreeNode::to_vec(vault_whitelist_metas);

        let hashed_nodes: Vec<[u8; 32]> = tree_nodes.iter().map(|n| n.hash().to_bytes()).collect();

        let mut index = 0;
        for (node_index, tree_node) in tree_nodes.iter().enumerate() {
            if tree_node.depositor.eq(depositor) {
                index = node_index;
            }
        }

        let merkle_tree = MerkleTree::new(&hashed_nodes[..], true);

        let path = merkle_tree.find_path(index).expect("path to index");

        for branch in path.get_proof_entries() {
            if let Some(hash) = branch.get_left_sibling() {
                proof.push(hash.to_bytes());
            } else if let Some(hash) = branch.get_right_sibling() {
                proof.push(hash.to_bytes());
            } else {
                panic!("expected some hash at each level of the tree");
            }
        }
        proof
    }
}
