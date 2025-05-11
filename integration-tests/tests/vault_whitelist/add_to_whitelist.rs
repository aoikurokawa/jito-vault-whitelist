#[cfg(test)]
mod tests {
    use jito_vault_sdk::error::VaultError;
    use jito_vault_whitelist_core::{whitelist::Whitelist, whitelist_user::WhitelistUser};
    use solana_sdk::pubkey::Pubkey;

    use crate::{client::vault_client::assert_vault_error, fixtures::fixture::TestBuilder};

    #[tokio::test]
    async fn test_add_to_whitelist() {
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

        let depositor = Pubkey::new_unique();

        vault_whitelist_client
            .do_add_to_whitelist(&vault_root, &depositor)
            .await
            .unwrap();

        let whitelist_pubkey = Whitelist::find_program_address(
            &jito_vault_whitelist_program::id(),
            &vault_root.vault_pubkey,
        )
        .0;
        let whitelist_user_pubkey = WhitelistUser::find_program_address(
            &jito_vault_whitelist_program::id(),
            &whitelist_pubkey,
            &depositor,
        )
        .0;
        let whitelist_user = vault_whitelist_client
            .get_whitelist_user(&whitelist_user_pubkey)
            .await
            .unwrap();

        assert_eq!(whitelist_user.whitelist, whitelist_pubkey);
        assert_eq!(whitelist_user.user, depositor);
    }

    #[tokio::test]
    async fn test_set_meta_merkle_root_invalid_vault_admin_fails() {
        let fixture = TestBuilder::new().await;
        let mut vault_program_client = fixture.vault_program_client();
        vault_program_client.do_initialize_config().await.unwrap();
        let mut vault_root_a = vault_program_client
            .do_initialize_vault(1000, 1000, 1000, 9, &Pubkey::new_unique())
            .await
            .unwrap();

        let mut vault_whitelist_client = fixture.vault_whitelist_program_client();
        vault_whitelist_client.do_initialize_config().await.unwrap();

        vault_whitelist_client
            .do_initialize_whitelist(&vault_root_a)
            .await
            .unwrap();

        let vault_root_b = vault_program_client
            .do_initialize_vault(1000, 1000, 1000, 9, &Pubkey::new_unique())
            .await
            .unwrap();
        vault_root_a.vault_admin = vault_root_b.vault_admin;

        let depositor = Pubkey::new_unique();

        let result = vault_whitelist_client
            .do_add_to_whitelist(&vault_root_a, &depositor)
            .await;

        assert_vault_error(result, VaultError::VaultAdminInvalid);
    }
}
