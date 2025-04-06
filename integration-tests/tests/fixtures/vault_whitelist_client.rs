use anchor_lang::AccountDeserialize;
use jito_vault_whitelist_client::accounts::Whitelist;
use solana_program_test::BanksClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

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
        Ok(*Whitelist::try_deserialize(&mut account.data.as_slice())?)
    }

    pub async fn do_initialize_whitelist(
        &mut self,
        vault: &Pubkey,
        meta_merkle_root: &[u8; 32],
    ) -> TestResult<Keypair> {
        let whitelist_pubkey =
            Whitelist::find_program_address(&vault_whitelist_program::id(), vault).0;
        let admin = Keypair::new();

        self._airdrop(&admin.pubkey(), 1.0).await?;
        self.initialize_whitelist(&whitelist_pubkey, &admin, root)
            .await?;

        Ok(admin)
    }
}
