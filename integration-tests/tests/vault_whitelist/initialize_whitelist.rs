#[cfg(test)]
mod tests {
    use solana_sdk::pubkey::Pubkey;

    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_initialize_whitelist() {
        let fixture = TestBuilder::new().await;
        let mut vault_whitelist_client = fixture.vault_whitelist_program_client();

        let vault = Pubkey::new_unique();
        let meta_merkle_root = [0; 32];

        vault_whitelist_client.do_initialize_config().await.unwrap();

        vault_whitelist_client
            .do_initialize_whitelist(vault, &meta_merkle_root)
            .await
            .unwrap();
    }
}
