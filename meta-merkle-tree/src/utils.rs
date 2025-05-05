use crate::{error::MerkleTreeError, merkle_tree::MerkleTree};

/// Extracts a Merkle proof from a MerkleTree for a specific leaf index.
///
/// This utility function traverses the path from a leaf node to the root
/// in a Merkle tree and collects all sibling hashes along this path.
/// These sibling hashes form the proof that can be used to verify the
/// inclusion of the leaf in the Merkle tree.
///
/// # Examples
///
/// ```
/// use jito_vault_whitelist_meta_merkle_tree::{merkle_tree::MerkleTree, utils::get_proof};
///
/// let data = vec!["a", "b", "c", "d"];
/// let merkle_tree = MerkleTree::new(&data, false);
///
/// let proof = get_proof(&merkle_tree, 1);
///
/// assert!(proof.is_ok());
/// ```
pub fn get_proof(merkle_tree: &MerkleTree, index: usize) -> Result<Vec<[u8; 32]>, MerkleTreeError> {
    let mut proof = Vec::new();
    let path = merkle_tree.find_path(index).expect("path to index");

    // Extract the sibling hashes along the path
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
