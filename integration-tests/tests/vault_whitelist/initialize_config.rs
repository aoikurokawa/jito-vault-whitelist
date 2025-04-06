#[cfg(test)]
mod tests {
    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_initialize_config() {
        let fixture = TestBuilder::new().await;
        let mut vault_whitelist_client = fixture.vault_whitelist_program_client();
        let mut vault_program_client = fixture.vault_program_client();

        vault_program_client.do_initialize_config().await.unwrap();

        vault_whitelist_client.do_initialize_config().await.unwrap();
    }
}
