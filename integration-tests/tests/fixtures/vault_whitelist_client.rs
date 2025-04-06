use jito_vault_whitelist_core::whitelist::Whitelist;
use solana_program_test::BanksClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

use super::TestResult;

pub struct VaultWhitelistProgramClient {
    banks_client: BanksClient,
    payer: Keypair,
}

impl VaultWhitelistProgramClient {
    pub fn new(banks_client: BanksClient, payer: Keypair) -> Self {
        Self {
            banks_client,
            payer,
        }
    }

    pub async fn get_whitelist(&mut self, account: &Pubkey) -> TestResult<Whitelist> {
        let account = self.banks_client.get_account(*account).await?.unwrap();
        Ok(*Whitelist::try_from_slice_unchecke(
            account.data.as_slice(),
        )?)
    }

    pub async fn do_initialize_whitelist(&mut self, root: &[u8; 32]) -> TestResult<Keypair> {
        let whitelist_pubkey = Whitelist::find_program_address(&ncn_portal_program::id()).0;
        let admin = Keypair::new();

        self._airdrop(&admin.pubkey(), 1.0).await?;
        self.initialize_whitelist(&whitelist_pubkey, &admin, root)
            .await?;

        Ok(admin)
    }
}
