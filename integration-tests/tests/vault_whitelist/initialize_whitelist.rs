#[cfg(test)]
mod tests {
    use jito_vault_sdk::error::VaultError;
    use jito_vault_whitelist_core::whitelist::Whitelist;
    use solana_sdk::pubkey::Pubkey;

    use crate::{client::vault_client::assert_vault_error, fixtures::fixture::TestBuilder};

    #[tokio::test]
    async fn test_initialize_whitelist() {
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

        let whitelist_pubkey = Whitelist::find_program_address(
            &jito_vault_whitelist_program::id(),
            &vault_root.vault_pubkey,
        )
        .0;
        let whitelist = vault_whitelist_client
            .get_whitelist(&whitelist_pubkey)
            .await
            .unwrap();

        assert_eq!(whitelist.vault, vault_root.vault_pubkey);
        assert_eq!(whitelist.meta_merkle_root, meta_merkle_root);
    }

    #[tokio::test]
    async fn test_initialize_whitelist_invalid_vault_admin_fails() {
        let fixture = TestBuilder::new().await;
        let mut vault_program_client = fixture.vault_program_client();
        vault_program_client.do_initialize_config().await.unwrap();
        let mut vault_root_a = vault_program_client
            .do_initialize_vault(1000, 1000, 1000, 9, &Pubkey::new_unique())
            .await
            .unwrap();

        let mut vault_whitelist_client = fixture.vault_whitelist_program_client();
        vault_whitelist_client.do_initialize_config().await.unwrap();

        let meta_merkle_root = [0; 32];

        let vault_root_b = vault_program_client
            .do_initialize_vault(1000, 1000, 1000, 9, &Pubkey::new_unique())
            .await
            .unwrap();
        vault_root_a.vault_admin = vault_root_b.vault_admin;

        let result = vault_whitelist_client
            .do_initialize_whitelist(&vault_root_a, &meta_merkle_root)
            .await;

        assert_vault_error(result, VaultError::VaultAdminInvalid);
    }
}
