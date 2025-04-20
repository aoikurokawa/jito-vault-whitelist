// Mostly copied from modules in jito-solana/tip-distributor/src
// To be replaced by tip distributor code in this repo

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

    // #[serde(with = "pubkey_string_conversion")]
    // pub merkle_root_upload_authority: Pubkey,
    pub merkle_root: Hash,

    pub tree_nodes: Vec<VaultWhitelistMetaTreeNode>,

    // pub max_total_claim: u64,
    pub max_num_nodes: u64,
}

impl GeneratedMerkleTree {
    pub fn new(user_account: &Pubkey, vault_whitelist_metas: &[VaultWhitelistMeta]) -> Self {
        let mut tree_nodes = VaultWhitelistMetaTreeNode::to_vec(vault_whitelist_metas);

        // for vault_whitelist_meta in vault_whitelist_metas {
        //     if let Ok(tree_nodes) =
        //         VaultWhitelistMetaTreeNode::new(&vault_whitelist_meta)
        //     {
        let hashed_nodes: Vec<[u8; 32]> = tree_nodes.iter().map(|n| n.hash().to_bytes()).collect();

        let merkle_tree = MerkleTree::new(&hashed_nodes[..], true);
        let max_num_nodes = tree_nodes.len() as u64;

        for (i, tree_node) in tree_nodes.iter_mut().enumerate() {
            tree_node.proof = Some(get_proof(&merkle_tree, i));
        }
        // }
        // }

        // if let Some(rpc_client) = &maybe_rpc_client {
        //     if let Some(tda) = stake_meta.maybe_tip_distribution_meta.as_ref() {
        // emit_inconsistent_tree_node_amount_dp(
        //     &tree_nodes[..],
        //     &tda.tip_distribution_pubkey,
        //     rpc_client,
        // );
        //     }
        // }

        // let tip_distribution_meta = stake_meta.maybe_tip_distribution_meta.unwrap();

        Self {
            max_num_nodes,
            user_account: *user_account,
            // tip_distribution_account: tip_distribution_meta.tip_distribution_pubkey,
            // merkle_root_upload_authority: tip_distribution_meta
            //     .merkle_root_upload_authority,
            merkle_root: *merkle_tree.get_root().unwrap(),
            tree_nodes,
            // max_total_claim: tip_distribution_meta.total_tips,
        }
    }
}
