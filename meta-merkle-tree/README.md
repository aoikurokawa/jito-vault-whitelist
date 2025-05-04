# Merkle Tree Whitelist

A crate for creating and verifying Merkle trees for whitelist verfication.
This implementation is efficiently manage whitelists for minting rights, and other access control mechanisms.

## Overview

This create provides implementation of Merkle tree for use in Jito Vault Whitelist program.
It enables efficient on-chain verification of whitelist membership by storing only a single Merkle root on-chain, while keeping the full whitelist off-chain.
Users can prove their inclusion in the whitelist by providing a Merkle proof.

## Usage Examples

### Creating a Merkle Tree from a Whtielist

```rust
// Load whitelist data from a JSON file
let whitelist_file_path = PathBuf::from("whitelist.json");
let vault_whitelist_metas = 
    read_json_from_file::<Vec<VaultWhitelistMeta>>(&whitelist_file_path)?;

// Create a new Merkle tree with the whitelist data
let merkle_tree = GeneratedMerkleTree::new(&vault_whitelist_metas);

// Get the Merkle root to store on-chain
let merkle_root = merkle_tree.merkle_root.to_bytes();
```

### Generating a Proof for Verfication

```rust
let whitelist_file_path = PathBuf::from("whitelist.json");
let vault_whitelist_metas = 
    read_json_from_file::<Vec<VaultWhitelistMeta>>(&whitelist_file_path)?;

// Get the user's public key
let user = Pubkey::from_str("BxZJkHu9xMvGzXFw83RXBRVgSMbHA8Z239R1svUBZWhx")?;

// Generate the Merkle proof for this depositor
let proof = GeneratedMerkleTree::get_proof(&vault_whitelist_metas, &depositor);
```
