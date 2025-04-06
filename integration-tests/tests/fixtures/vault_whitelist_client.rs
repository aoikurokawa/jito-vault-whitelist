use anchor_lang::AccountDeserialize;
use jito_vault_whitelist_client::instructions::InitializeWhitelistBuilder;
use jito_vault_whitelist_core::{config::Config, whitelist::Whitelist};
use solana_program_test::BanksClient;
use solana_sdk::{
    commitment_config::CommitmentLevel, native_token::sol_to_lamports, pubkey::Pubkey,
    signature::Keypair, signer::Signer, system_instruction::transfer, transaction::Transaction,
};

use super::TestResult;

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

    pub async fn _airdrop(&mut self, to: &Pubkey, sol: f64) -> TestResult<()> {
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

    // pub async fn do_initialize_config(&mut self) -> TestResult<Keypair> {
    //     let restaking_config_pubkey =
    //         Config::find_program_address(&jito_vault_whitelist_program::id()).0;
    //     let restaking_config_admin = Keypair::new();

    //     self._airdrop(&restaking_config_admin.pubkey(), 1.0).await?;
    //     self.initialize_config(&restaking_config_pubkey, &restaking_config_admin)
    //         .await?;

    //     Ok(restaking_config_admin)
    // }

    // pub async fn initialize_config(
    //     &mut self,
    //     config: &Pubkey,
    //     config_admin: &Keypair,
    // ) -> TestResult<()> {
    //     let blockhash = self.banks_client.get_latest_blockhash().await?;
    //     self.process_transaction(&Transaction::new_signed_with_payer(
    //         &[initialize_config(
    //             &jito_restaking_program::id(),
    //             config,
    //             &config_admin.pubkey(),
    //             &jito_vault_program::id(),
    //         )],
    //         Some(&config_admin.pubkey()),
    //         &[config_admin],
    //         blockhash,
    //     ))
    //     .await
    // }

    pub async fn do_initialize_whitelist(
        &mut self,
        vault: Pubkey,
        meta_merkle_root: &[u8; 32],
    ) -> TestResult<Keypair> {
        let whitelist_pubkey =
            Whitelist::find_program_address(&jito_vault_whitelist_program::id(), &vault).0;
        let admin = Keypair::new();

        self._airdrop(&admin.pubkey(), 1.0).await?;
        self.initialize_whitelist(vault, meta_merkle_root).await?;

        Ok(admin)
    }

    pub async fn initialize_whitelist(
        &mut self,
        vault: Pubkey,
        meta_merkle_root: &[u8; 32],
    ) -> TestResult<()> {
        let config = Config::find_program_address(&jito_vault_whitelist_program::id(), &vault).0;

        let ix = InitializeWhitelistBuilder::new()
            .config(config)
            .vault(vault)
            .meta_merkle_root(*meta_merkle_root)
            .instruction();

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
