#[cfg(test)]
mod tests {
    use solana_sdk::pubkey::Pubkey;

    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_initialize_whitelist() {
        let fixture = TestBuilder::new().await;
        let mut vault_program_client = fixture.vault_program_client();
        vault_program_client.do_initialize_config().await.unwrap();

        vault_program_client
            .do_initialize_vault(1000, 1000, 1000, 9, &Pubkey::new_unique(), None)
            .await
            .unwrap();

        let mut vault_whitelist_client = fixture.vault_whitelist_program_client();
        vault_whitelist_client.do_initialize_config().await.unwrap();

        let vault = Pubkey::new_unique();
        let meta_merkle_root = [0; 32];

        vault_whitelist_client
            .do_initialize_whitelist(vault, &meta_merkle_root)
            .await
            .unwrap();
    }
}
