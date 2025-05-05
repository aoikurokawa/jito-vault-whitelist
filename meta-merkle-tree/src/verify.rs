use solana_program::hash::hashv;

/// Verfies a Merkle proof against a know root hash.
///
/// This function verifies that a leaf node is included in a Merkle tree by checking
/// its proof against the tree's root hash. It protects against second preimage attacks
/// by using a prefix byte (0x01) for intermediate nodes.
///
/// This is a modified version of the verification algorithm from the Saber Merkle Distributor:
/// https://github.com/saber-hq/merkle-distributor/blob/ac937d1901033ecb7fa3b0db22f7b39569c8e052/programs/merkle-distributor/src/merkle_proof.rs#L8
///
/// Originally ported from OpenZeppelin's MerkleProof.sol:
/// Direct port of https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v3.4.0/contracts/cryptography/MerkleProof.sol
///
/// # Returns
///
/// `true` if a `leaf` can be proved to be a part of a Merkle tree, `false` otherwise
pub fn verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter() {
        if computed_hash <= proof_element {
            // Hash(current computed hash + current element of the proof)
            computed_hash = hashv(&[&[1u8], &computed_hash, &proof_element]).to_bytes();
        } else {
            // Hash(current element of the proof + current computed hash)
            computed_hash = hashv(&[&[1u8], &proof_element, &computed_hash]).to_bytes();
        }
    }
    // Check if the computed hash (root) is equal to the provided root
    computed_hash == root
}
