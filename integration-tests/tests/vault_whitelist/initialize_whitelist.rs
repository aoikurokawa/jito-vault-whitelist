#[cfg(test)]
mod tests {
    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_initialize_whitelist() {
        let mut fixture = TestBuilder::new().await;
        let mut vault_whitelist_client = fixture.vault_whitelist_program_client();
    }
}
