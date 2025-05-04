use serde::{Deserialize, Serialize};
use solana_program::hash::{hashv, Hash};

use crate::generated_merkle_tree::GeneratedMerkleTree;

/// Represents the information for activating a tip distribution account.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TreeNode {
    /// Claimant's proof of inclusion in the Merkle Tree
    pub proof: Option<Vec<[u8; 32]>>,

    /// Number of nodes to claim
    pub max_num_nodes: u64,
}

impl TreeNode {
    pub const fn new(max_num_nodes: u64) -> Self {
        Self {
            proof: None,
            max_num_nodes,
        }
    }

    pub fn hash(&self) -> Hash {
        hashv(&[&self.max_num_nodes.to_le_bytes()])
    }
}

impl From<GeneratedMerkleTree> for TreeNode {
    fn from(generated_merkle_tree: GeneratedMerkleTree) -> Self {
        Self {
            max_num_nodes: generated_merkle_tree.max_num_nodes,
            proof: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_tree_node() {
        let tree_node = TreeNode {
            proof: None,
            max_num_nodes: 0,
        };
        let serialized = serde_json::to_string(&tree_node).unwrap();
        let deserialized: TreeNode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tree_node, deserialized);
    }
}
