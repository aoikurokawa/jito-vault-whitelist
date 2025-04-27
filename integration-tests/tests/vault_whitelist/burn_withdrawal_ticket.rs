#[cfg(test)]
mod tests {
    use meta_merkle_tree::{
        generated_merkle_tree::GeneratedMerkleTree, vault_whitelist_meta::VaultWhitelistMeta,
    };
    use solana_sdk::{signature::Keypair, signer::Signer};

    use crate::{
        client::vault_client::VaultStakerWithdrawalTicketRoot,
        fixtures::fixture::{ConfiguredVault, TestBuilder},
    };

    #[tokio::test]
    async fn test_enqueue_withdrawal() {
        const MINT_AMOUNT: u64 = 100_000;
        const DEPOSIT_FEE_BPS: u16 = 100;
        const WITHDRAWAL_FEE_BPS: u16 = 100;
        // let min_amount_out: u64 = MINT_AMOUNT * (10_000 - DEPOSIT_FEE_BPS) as u64 / 10_000;

        let deposit_fee_bps = DEPOSIT_FEE_BPS;
        let withdrawal_fee_bps = WITHDRAWAL_FEE_BPS;
        let reward_fee_bps = 0;
        let num_operators = 1;
        let slasher_amounts = vec![];

        let mut fixture = TestBuilder::new().await;
        let ConfiguredVault {
            mut vault_program_client,
            mut vault_whitelist_client,
            vault_root,
            operator_roots,
            ..
        } = fixture
            .setup_vault_with_ncn_and_operators(
                deposit_fee_bps,
                withdrawal_fee_bps,
                reward_fee_bps,
                num_operators,
                &slasher_amounts,
            )
            .await
            .unwrap();

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

        // let vault_vrt_account = fixture
        //     .get_token_account(&get_associated_token_address(
        //         &depositor.pubkey(),
        //         &vault.vrt_mint,
        //     ))
        //     .await
        //     .unwrap();
        // assert_eq!(
        //     vault_vrt_account.amount,
        //     MINT_AMOUNT * (10_000 - DEPOSIT_FEE_BPS) as u64 / 10_000
        // );

        // let vault_fee_account = fixture
        //     .get_token_account(&get_associated_token_address(
        //         &vault.fee_wallet,
        //         &vault.vrt_mint,
        //     ))
        //     .await
        //     .unwrap();
        // assert_eq!(
        //     vault_fee_account.amount,
        //     MINT_AMOUNT * DEPOSIT_FEE_BPS as u64 / 10_000
        // );

        // let vault operator ticket warmup
        let config = vault_program_client
            .get_config(
                &jito_vault_core::config::Config::find_program_address(&jito_vault_program::id()).0,
            )
            .await
            .unwrap();
        fixture
            .warp_slot_incremental(2 * config.epoch_length())
            .await
            .unwrap();

        let operator_root_pubkeys: Vec<_> = operator_roots
            .iter()
            .map(|root| root.operator_pubkey)
            .collect();
        vault_program_client
            .do_full_vault_update(&vault_root.vault_pubkey, &operator_root_pubkeys)
            .await
            .unwrap();

        let operator_root = operator_roots.first().unwrap();
        vault_program_client
            .do_add_delegation(&vault_root, &operator_root.operator_pubkey, MINT_AMOUNT)
            .await
            .unwrap();

        let vault_operator_delegation = vault_program_client
            .get_vault_operator_delegation(&vault_root.vault_pubkey, &operator_root.operator_pubkey)
            .await
            .unwrap();
        assert_eq!(
            vault_operator_delegation.delegation_state.staked_amount(),
            MINT_AMOUNT
        );

        // the user is withdrawing 99,000 VRT tokens, there is a 1% fee on withdraws, so
        // 98010 tokens will be undeleged for withdraw
        let amount_to_dequeue = MINT_AMOUNT * (10_000 - WITHDRAWAL_FEE_BPS) as u64 / 10_000;

        let vault = vault_program_client
            .get_vault(&vault_root.vault_pubkey)
            .await
            .unwrap();

        let vault_whitelist_metas = vec![VaultWhitelistMeta {
            depositor_pubkey: depositor.pubkey(),
        }];

        let proof = GeneratedMerkleTree::get_proof(&vault_whitelist_metas, &depositor.pubkey());

        let VaultStakerWithdrawalTicketRoot { base } = vault_whitelist_client
            .do_enqueue_withdrawal(&vault_root, &vault, &depositor, &proof, amount_to_dequeue)
            .await
            .unwrap();

        vault_program_client
            .do_cooldown_delegation(&vault_root, &operator_roots[0].operator_pubkey, MINT_AMOUNT)
            .await
            .unwrap();

        fixture
            .warp_slot_incremental(config.epoch_length())
            .await
            .unwrap();
        vault_program_client
            .do_full_vault_update(
                &vault_root.vault_pubkey,
                &[operator_roots[0].operator_pubkey],
            )
            .await
            .unwrap();
        fixture
            .warp_slot_incremental(config.epoch_length())
            .await
            .unwrap();
        vault_program_client
            .do_full_vault_update(
                &vault_root.vault_pubkey,
                &[operator_roots[0].operator_pubkey],
            )
            .await
            .unwrap();

        vault_whitelist_client
            .do_burn_withdrawal_ticket(&config, &vault_root, &vault, &depositor, &base, &proof)
            .await
            .unwrap();
    }

    // #[tokio::test]
    // async fn test_mint_invalid_proof() {
    //     let fixture = TestBuilder::new().await;
    //     let mut vault_program_client = fixture.vault_program_client();
    //     vault_program_client.do_initialize_config().await.unwrap();
    //     let vault_root = vault_program_client
    //         .do_initialize_vault(1000, 1000, 1000, 9, &Pubkey::new_unique(), None)
    //         .await
    //         .unwrap();

    //     let mut vault_whitelist_client = fixture.vault_whitelist_program_client();
    //     vault_whitelist_client.do_initialize_config().await.unwrap();

    //     let meta_merkle_root = [0; 32];

    //     vault_whitelist_client
    //         .do_initialize_whitelist(&vault_root, &meta_merkle_root)
    //         .await
    //         .unwrap();

    //     vault_whitelist_client
    //         .do_set_mint_burn_admin(&vault_root)
    //         .await
    //         .unwrap();

    //     let vault = vault_program_client
    //         .get_vault(&vault_root.vault_pubkey)
    //         .await
    //         .unwrap();

    //     let depositor = Keypair::new();
    //     vault_program_client
    //         .configure_depositor(&vault_root, &depositor.pubkey(), MINT_AMOUNT)
    //         .await
    //         .unwrap();

    //     let vault_whitelist_metas = vec![VaultWhitelistMeta {
    //         depositor_pubkey: depositor.pubkey(),
    //     }];

    //     let merkle_tree =
    //         GeneratedMerkleTree::new(&vault_root.vault_admin.pubkey(), &vault_whitelist_metas);

    //     vault_whitelist_client
    //         .do_set_meta_merkle_root(&vault_root, &merkle_tree.merkle_root.to_bytes())
    //         .await
    //         .unwrap();

    //     let proof = GeneratedMerkleTree::get_proof(&vault_whitelist_metas, &depositor.pubkey());

    //     let min_amount_out: u64 = 90000;

    //     let invalid_depositor = Keypair::new();
    //     vault_program_client
    //         .configure_depositor(&vault_root, &invalid_depositor.pubkey(), MINT_AMOUNT)
    //         .await
    //         .unwrap();

    //     let result = vault_whitelist_client
    //         .do_mint(
    //             &vault_root,
    //             &vault,
    //             &invalid_depositor,
    //             &proof,
    //             MINT_AMOUNT,
    //             min_amount_out,
    //         )
    //         .await;

    //     assert_vault_whitelist_error(result, VaultWhitelistError::InvalidProof);
    // }
}
