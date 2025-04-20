use serde::{Deserialize, Serialize};

use crate::{
    error::MerkleRootGeneratorError, generated_merkle_tree::GeneratedMerkleTree,
    merkle_tree::MerkleTree, utils::get_proof,
    vault_whitelist_meta_collection::VaultWhitelistMetaCollection,
    vault_whitelist_meta_tree_node::VaultWhitelistMetaTreeNode,
};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GeneratedMerkleTreeCollection {
    pub generated_merkle_trees: Vec<GeneratedMerkleTree>,
    // pub bank_hash: String,
    // pub epoch: Epoch,
    // pub slot: Slot,
}

impl GeneratedMerkleTreeCollection {
    pub fn new_from_stake_meta_collection(
        vault_whitelist_meta_coll: VaultWhitelistMetaCollection,
    ) -> Result<Self, MerkleRootGeneratorError> {
        let generated_merkle_trees = vault_whitelist_meta_coll
            .vault_whitelist_metas
            .into_iter()
            // .filter(|stake_meta| stake_meta.maybe_tip_distribution_meta.is_some())
            .filter_map(|vault_whitelist_meta| {
                let mut tree_nodes =
                    match VaultWhitelistMetaTreeNode::vec_from_stake_meta(&vault_whitelist_meta) {
                        Ok(maybe_tree_nodes) => maybe_tree_nodes,
                        Err(e) => return Some(Err(e)),
                    };

                let hashed_nodes: Vec<[u8; 32]> =
                    tree_nodes.iter().map(|n| n.hash().to_bytes()).collect();

                // let tip_distribution_meta = stake_meta.maybe_tip_distribution_meta.unwrap();

                let merkle_tree = MerkleTree::new(&hashed_nodes[..], true);
                let max_num_nodes = tree_nodes.len() as u64;

                for (i, tree_node) in tree_nodes.iter_mut().enumerate() {
                    tree_node.proof = Some(get_proof(&merkle_tree, i));
                }

                Some(Ok(GeneratedMerkleTree {
                    max_num_nodes,
                    user_account: vault_whitelist_meta.depositor_pubkey,
                    // tip_distribution_account: tip_distribution_meta.tip_distribution_pubkey,
                    // merkle_root_upload_authority: tip_distribution_meta
                    //     .merkle_root_upload_authority,
                    merkle_root: *merkle_tree.get_root().unwrap(),
                    tree_nodes,
                    // max_total_claim: tip_distribution_meta.total_tips,
                }))
            })
            .collect::<Result<Vec<GeneratedMerkleTree>, MerkleRootGeneratorError>>()?;

        Ok(Self {
            generated_merkle_trees,
            // bank_hash: stake_meta_coll.bank_hash,
            // epoch: stake_meta_coll.epoch,
            // slot: stake_meta_coll.slot,
        })
    }
}
