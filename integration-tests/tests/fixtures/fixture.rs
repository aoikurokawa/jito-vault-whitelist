use std::fmt::{Debug, Formatter};

use solana_program_test::{processor, ProgramTest, ProgramTestContext};

use crate::client::{
    vault_client::VaultProgramClient, vault_whitelist_client::VaultWhitelistClient,
};

pub struct TestBuilder {
    pub context: ProgramTestContext,
}

impl Debug for TestBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestBuilder",)
    }
}

impl TestBuilder {
    pub async fn new() -> Self {
        // $ cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run
        let mut program_test = ProgramTest::new(
            "jito_vault_whitelist_program",
            jito_vault_whitelist_program::id(),
            processor!(jito_vault_whitelist_program::process_instruction),
        );
        program_test.prefer_bpf(true);
        program_test.add_program("jito_vault_program", jito_vault_program::id(), None);

        let context = program_test.start_with_context().await;
        Self { context }
    }

    pub fn vault_program_client(&self) -> VaultProgramClient {
        VaultProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    pub fn vault_whitelist_program_client(&self) -> VaultWhitelistClient {
        VaultWhitelistClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }
}
