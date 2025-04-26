#[cfg(test)]
mod tests {
    use jito_vault_whitelist_sdk::error::VaultWhitelistError;
    use meta_merkle_tree::{
        generated_merkle_tree::GeneratedMerkleTree, vault_whitelist_meta::VaultWhitelistMeta,
    };
    use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

    use crate::{
        client::vault_whitelist_client::assert_vault_whitelist_error,
        fixtures::fixture::TestBuilder,
    };

    const MINT_AMOUNT: u64 = 100_000;

    #[tokio::test]
    async fn test_mint() {
        let fixture = TestBuilder::new().await;
        let mut vault_program_client = fixture.vault_program_client();
        vault_program_client.do_initialize_config().await.unwrap();
        let vault_root = vault_program_client
            .do_initialize_vault(1000, 1000, 1000, 9, &Pubkey::new_unique())
            .await
            .unwrap();

        let mut vault_whitelist_client = fixture.vault_whitelist_program_client();
        vault_whitelist_client.do_initialize_config().await.unwrap();

        let meta_merkle_root = [0; 32];

        vault_whitelist_client
            .do_initialize_whitelist(&vault_root, &meta_merkle_root)
            .await
            .unwrap();

        vault_whitelist_client
            .do_set_mint_burn_admin(&vault_root)
            .await
            .unwrap();

        let vault = vault_program_client
            .get_vault(&vault_root.vault_pubkey)
            .await
            .unwrap();

        let depositor = Keypair::new();
        vault_program_client
            .configure_depositor(&vault_root, &depositor.pubkey(), MINT_AMOUNT)
            .await
            .unwrap();

        let vault_whitelist_metas = vec![VaultWhitelistMeta {
            depositor_pubkey: depositor.pubkey(),
        }];

        let merkle_tree =
            GeneratedMerkleTree::new(&vault_root.vault_admin.pubkey(), &vault_whitelist_metas);

        vault_whitelist_client
            .do_set_meta_merkle_root(&vault_root, &merkle_tree.merkle_root.to_bytes())
            .await
            .unwrap();

        let proof = GeneratedMerkleTree::get_proof(&vault_whitelist_metas, &depositor.pubkey());

        let min_amount_out: u64 = 90000;

        vault_whitelist_client
            .do_mint(
                &vault_root,
                &vault,
                &depositor,
                &proof,
                MINT_AMOUNT,
                min_amount_out,
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_mint_invalid_proof() {
        let fixture = TestBuilder::new().await;
        let mut vault_program_client = fixture.vault_program_client();
        vault_program_client.do_initialize_config().await.unwrap();
        let vault_root = vault_program_client
            .do_initialize_vault(1000, 1000, 1000, 9, &Pubkey::new_unique())
            .await
            .unwrap();

        let mut vault_whitelist_client = fixture.vault_whitelist_program_client();
        vault_whitelist_client.do_initialize_config().await.unwrap();

        let meta_merkle_root = [0; 32];

        vault_whitelist_client
            .do_initialize_whitelist(&vault_root, &meta_merkle_root)
            .await
            .unwrap();

        vault_whitelist_client
            .do_set_mint_burn_admin(&vault_root)
            .await
            .unwrap();

        let vault = vault_program_client
            .get_vault(&vault_root.vault_pubkey)
            .await
            .unwrap();

        let depositor = Keypair::new();
        vault_program_client
            .configure_depositor(&vault_root, &depositor.pubkey(), MINT_AMOUNT)
            .await
            .unwrap();

        let vault_whitelist_metas = vec![VaultWhitelistMeta {
            depositor_pubkey: depositor.pubkey(),
        }];

        let merkle_tree =
            GeneratedMerkleTree::new(&vault_root.vault_admin.pubkey(), &vault_whitelist_metas);

        vault_whitelist_client
            .do_set_meta_merkle_root(&vault_root, &merkle_tree.merkle_root.to_bytes())
            .await
            .unwrap();

        let proof = GeneratedMerkleTree::get_proof(&vault_whitelist_metas, &depositor.pubkey());

        let min_amount_out: u64 = 90000;

        let invalid_depositor = Keypair::new();
        vault_program_client
            .configure_depositor(&vault_root, &invalid_depositor.pubkey(), MINT_AMOUNT)
            .await
            .unwrap();

        let result = vault_whitelist_client
            .do_mint(
                &vault_root,
                &vault,
                &invalid_depositor,
                &proof,
                MINT_AMOUNT,
                min_amount_out,
            )
            .await;

        assert_vault_whitelist_error(result, VaultWhitelistError::InvalidProof);
    }
}
