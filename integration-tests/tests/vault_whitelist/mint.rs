#[cfg(test)]
mod tests {
    use solana_sdk::{
        instruction::InstructionError, pubkey::Pubkey, signature::Keypair, signer::Signer,
    };

    use crate::fixtures::{assert_ix_error, fixture::TestBuilder};

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

        vault_whitelist_client
            .do_initialize_whitelist(&vault_root)
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

        vault_whitelist_client
            .do_add_to_whitelist(&vault_root, &depositor.pubkey())
            .await
            .unwrap();

        let min_amount_out: u64 = 90000;

        vault_whitelist_client
            .do_mint(&vault_root, &vault, &depositor, MINT_AMOUNT, min_amount_out)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_mint_invalid_user() {
        let fixture = TestBuilder::new().await;
        let mut vault_program_client = fixture.vault_program_client();
        vault_program_client.do_initialize_config().await.unwrap();
        let vault_root = vault_program_client
            .do_initialize_vault(1000, 1000, 1000, 9, &Pubkey::new_unique())
            .await
            .unwrap();

        let mut vault_whitelist_client = fixture.vault_whitelist_program_client();
        vault_whitelist_client.do_initialize_config().await.unwrap();

        vault_whitelist_client
            .do_initialize_whitelist(&vault_root)
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

        vault_whitelist_client
            .add_to_whitelist(&vault_root, &depositor.pubkey())
            .await
            .unwrap();

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
                MINT_AMOUNT,
                min_amount_out,
            )
            .await;

        assert_ix_error(result, InstructionError::InvalidAccountOwner);
    }
}
