use serde::{Deserialize, Serialize};
use solana_program::hash::Hash;
use solana_program::pubkey::Pubkey;

use crate::error::MerkleTreeError;
use crate::merkle_tree::MerkleTree;
use crate::utils::get_proof;
use crate::vault_whitelist_meta::VaultWhitelistMeta;
use crate::vault_whitelist_meta_tree_node::VaultWhitelistMetaTreeNode;

/// A Merkle tree specifically designed for whitelisting depositors in a vault system.
///
/// This struct represents a Merkle tree built from a list of whitelisted vault depositors.
/// It includes both the tree's root hash (used for on-chain verification) and the complete
/// set of tree nodes with their respective proofs (used for off-chain proof generation).
#[derive(Clone, Eq, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct GeneratedMerkleTree {
    /// Merkle root
    pub merkle_root: Hash,

    /// Tree nodes
    pub tree_nodes: Vec<VaultWhitelistMetaTreeNode>,

    /// Maximum number of nodes
    pub max_num_nodes: u64,
}

impl GeneratedMerkleTree {
    /// Creates a new Merkle tree for a vault whitelist.
    ///
    /// This function builds a Merkle tree from a list of whitelisted vault users.
    /// It generates a tree where each leaf represents a whitelisted user, and each
    /// node includes a proof of inclusion that can be verified against the tree's root.
    pub fn new(vault_whitelist_metas: &[VaultWhitelistMeta]) -> Result<Self, MerkleTreeError> {
        let mut tree_nodes = VaultWhitelistMetaTreeNode::to_vec(vault_whitelist_metas);

        let hashed_nodes: Vec<[u8; 32]> = tree_nodes.iter().map(|n| n.hash().to_bytes()).collect();

        let merkle_tree = MerkleTree::new(&hashed_nodes[..], true);
        let max_num_nodes = tree_nodes.len() as u64;

        for (i, tree_node) in tree_nodes.iter_mut().enumerate() {
            let proof = get_proof(&merkle_tree, i)?;
            tree_node.proof = Some(proof);
        }

        Ok(Self {
            max_num_nodes,
            merkle_root: *merkle_tree.get_root().unwrap(),
            tree_nodes,
        })
    }

    /// Generates a Merkle proof for a specific depositor.
    ///
    /// This static method creates a proof that can be used to verify that a depositor
    /// is included in the whitelist, without needing access to the complete tree.
    pub fn get_proof(
        vault_whitelist_metas: &[VaultWhitelistMeta],
        depositor: &Pubkey,
    ) -> Result<Vec<[u8; 32]>, MerkleTreeError> {
        let mut proof = Vec::new();

        let tree_nodes = VaultWhitelistMetaTreeNode::to_vec(vault_whitelist_metas);

        let hashed_nodes: Vec<[u8; 32]> = tree_nodes.iter().map(|n| n.hash().to_bytes()).collect();

        let mut index = 0;
        for (node_index, tree_node) in tree_nodes.iter().enumerate() {
            if tree_node.user.eq(depositor) {
                index = node_index;
                break;
            }
        }

        let merkle_tree = MerkleTree::new(&hashed_nodes[..], true);

        let path = match merkle_tree.find_path(index) {
            Some(path) => path,
            None => {
                return Err(MerkleTreeError::MerkleValidationError(
                    "path to index".to_string(),
                ))
            }
        };

        for branch in path.get_proof_entries() {
            if let Some(hash) = branch.get_left_sibling() {
                proof.push(hash.to_bytes());
            } else if let Some(hash) = branch.get_right_sibling() {
                proof.push(hash.to_bytes());
            } else {
                return Err(MerkleTreeError::MerkleValidationError(
                    "expected some hash at each level of the tree".to_string(),
                ));
            }
        }

        Ok(proof)
    }
}
