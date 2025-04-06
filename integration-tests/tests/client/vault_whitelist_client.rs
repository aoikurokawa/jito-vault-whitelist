use anchor_lang::AccountDeserialize;
use jito_vault_whitelist_client::instructions::{
    InitializeConfigBuilder, InitializeWhitelistBuilder,
};
use jito_vault_whitelist_core::{config::Config, whitelist::Whitelist};
use solana_program_test::BanksClient;
use solana_sdk::{
    commitment_config::CommitmentLevel, native_token::sol_to_lamports, pubkey::Pubkey,
    signature::Keypair, signer::Signer, system_instruction::transfer, transaction::Transaction,
};

use crate::fixtures::TestResult;

pub struct VaultWhitelistClient {
    banks_client: BanksClient,

    payer: Keypair,
}

impl VaultWhitelistClient {
    pub fn new(banks_client: BanksClient, payer: Keypair) -> Self {
        Self {
            banks_client,
            payer,
        }
    }

    pub async fn process_transaction(&mut self, tx: &Transaction) -> TestResult<()> {
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                tx.clone(),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    pub async fn airdrop(&mut self, to: &Pubkey, sol: f64) -> TestResult<()> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(&self.payer.pubkey(), to, sol_to_lamports(sol))],
                    Some(&self.payer.pubkey()),
                    &[&self.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    pub async fn get_whitelist(
        &mut self,
        account: &Pubkey,
    ) -> TestResult<jito_vault_whitelist_client::accounts::Whitelist> {
        let account = self.banks_client.get_account(*account).await?.unwrap();
        let whitelist = jito_vault_whitelist_client::accounts::Whitelist::try_deserialize(
            &mut account.data.as_slice(),
        )
        .unwrap();

        Ok(whitelist)
    }

    pub async fn do_initialize_config(&mut self) -> TestResult<()> {
        self.airdrop(&self.payer.pubkey(), 100.0).await.unwrap();
        self.initialize_config().await?;

        Ok(())
    }

    pub async fn initialize_config(&mut self) -> TestResult<()> {
        let config = jito_vault_whitelist_core::config::Config::find_program_address(
            &jito_vault_whitelist_program::id(),
        )
        .0;

        let mut ix = InitializeConfigBuilder::new()
            .config(config)
            .admin(self.payer.pubkey())
            .instruction();
        ix.program_id = jito_vault_whitelist_program::id();

        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            blockhash,
        ))
        .await
    }

    pub async fn do_initialize_whitelist(
        &mut self,
        vault: Pubkey,
        meta_merkle_root: &[u8; 32],
    ) -> TestResult<()> {
        self.initialize_whitelist(vault, meta_merkle_root).await?;

        Ok(())
    }

    pub async fn initialize_whitelist(
        &mut self,
        vault: Pubkey,
        meta_merkle_root: &[u8; 32],
    ) -> TestResult<()> {
        let config = Config::find_program_address(&jito_vault_whitelist_program::id()).0;
        let whitelist =
            Whitelist::find_program_address(&jito_vault_whitelist_program::id(), &vault).0;

        let mut ix = InitializeWhitelistBuilder::new()
            .config(config)
            .whitelist(whitelist)
            .vault(vault)
            .vault_admin(self.payer.pubkey())
            .meta_merkle_root(*meta_merkle_root)
            .instruction();
        ix.program_id = jito_vault_whitelist_program::id();

        let blockhash = self.banks_client.get_latest_blockhash().await?;

        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            blockhash,
        ))
        .await
    }
}
