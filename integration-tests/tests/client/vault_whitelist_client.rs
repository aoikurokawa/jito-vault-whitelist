use anchor_lang::AccountDeserialize;
use jito_vault_core::vault::Vault;
use jito_vault_whitelist_client::instructions::{
    InitializeConfigBuilder, InitializeWhitelistBuilder, MintBuilder, SetMetaMerkleRootBuilder,
    SetMintBurnAdminBuilder,
};
use jito_vault_whitelist_core::{config::Config, whitelist::Whitelist};
use jito_vault_whitelist_sdk::error::VaultWhitelistError;
use solana_program_test::BanksClient;
use solana_sdk::{
    commitment_config::CommitmentLevel,
    instruction::InstructionError,
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction::transfer,
    transaction::{Transaction, TransactionError},
};
use spl_associated_token_account::get_associated_token_address;

use crate::fixtures::{TestError, TestResult};

use super::vault_client::VaultRoot;

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

    #[allow(dead_code)]
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
        vault_root: &VaultRoot,
        meta_merkle_root: &[u8; 32],
    ) -> TestResult<()> {
        self.initialize_whitelist(vault_root, meta_merkle_root)
            .await?;

        Ok(())
    }

    pub async fn initialize_whitelist(
        &mut self,
        vault_root: &VaultRoot,
        meta_merkle_root: &[u8; 32],
    ) -> TestResult<()> {
        let config = Config::find_program_address(&jito_vault_whitelist_program::id()).0;
        let whitelist = Whitelist::find_program_address(
            &jito_vault_whitelist_program::id(),
            &vault_root.vault_pubkey,
        )
        .0;

        let mut ix = InitializeWhitelistBuilder::new()
            .config(config)
            .whitelist(whitelist)
            .vault(vault_root.vault_pubkey)
            .vault_admin(vault_root.vault_admin.pubkey())
            .meta_merkle_root(*meta_merkle_root)
            .instruction();
        ix.program_id = jito_vault_whitelist_program::id();

        let blockhash = self.banks_client.get_latest_blockhash().await?;

        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&vault_root.vault_admin.pubkey()),
            &[&vault_root.vault_admin],
            blockhash,
        ))
        .await
    }

    pub async fn do_set_mint_burn_admin(&mut self, vault_root: &VaultRoot) -> TestResult<()> {
        self.set_mint_burn_admin(vault_root).await?;

        Ok(())
    }

    pub async fn set_mint_burn_admin(&mut self, vault_root: &VaultRoot) -> TestResult<()> {
        let config = Config::find_program_address(&jito_vault_whitelist_program::id()).0;
        let whitelist = Whitelist::find_program_address(
            &jito_vault_whitelist_program::id(),
            &vault_root.vault_pubkey,
        )
        .0;

        let mut ix = SetMintBurnAdminBuilder::new()
            .config(config)
            .vault_config(
                jito_vault_core::config::Config::find_program_address(&jito_vault_program::id()).0,
            )
            .whitelist(whitelist)
            .vault(vault_root.vault_pubkey)
            .vault_admin(vault_root.vault_admin.pubkey())
            .jito_vault_program(jito_vault_program::id())
            .instruction();
        ix.program_id = jito_vault_whitelist_program::id();

        let blockhash = self.banks_client.get_latest_blockhash().await?;

        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&vault_root.vault_admin.pubkey()),
            &[&vault_root.vault_admin],
            blockhash,
        ))
        .await
    }

    pub async fn do_set_meta_merkle_root(
        &mut self,
        vault_root: &VaultRoot,
        meta_merkle_root: &[u8; 32],
    ) -> TestResult<()> {
        self.set_meta_merkle_root(vault_root, meta_merkle_root)
            .await?;

        Ok(())
    }

    pub async fn set_meta_merkle_root(
        &mut self,
        vault_root: &VaultRoot,
        meta_merkle_root: &[u8; 32],
    ) -> TestResult<()> {
        let config = Config::find_program_address(&jito_vault_whitelist_program::id()).0;
        let whitelist = Whitelist::find_program_address(
            &jito_vault_whitelist_program::id(),
            &vault_root.vault_pubkey,
        )
        .0;

        let mut ix = SetMetaMerkleRootBuilder::new()
            .config(config)
            .whitelist(whitelist)
            .vault(vault_root.vault_pubkey)
            .vault_admin(vault_root.vault_admin.pubkey())
            .meta_merkle_root(*meta_merkle_root)
            .instruction();
        ix.program_id = jito_vault_whitelist_program::id();

        let blockhash = self.banks_client.get_latest_blockhash().await?;

        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&vault_root.vault_admin.pubkey()),
            &[&vault_root.vault_admin],
            blockhash,
        ))
        .await
    }

    pub async fn do_mint(
        &mut self,
        vault_root: &VaultRoot,
        vault: &Vault,
        depositor: &Keypair,
        proof: &[[u8; 32]],
        amount_in: u64,
        min_amount_out: u64,
    ) -> TestResult<()> {
        self.mint(
            &vault_root.vault_pubkey,
            &vault.vrt_mint,
            depositor,
            &get_associated_token_address(&depositor.pubkey(), &vault.supported_mint),
            &get_associated_token_address(&vault_root.vault_pubkey, &vault.supported_mint),
            &get_associated_token_address(&depositor.pubkey(), &vault.vrt_mint),
            &get_associated_token_address(&vault.fee_wallet, &vault.vrt_mint),
            proof,
            amount_in,
            min_amount_out,
        )
        .await?;

        Ok(())
    }

    pub async fn mint(
        &mut self,
        vault_pubkey: &Pubkey,
        vrt_mint: &Pubkey,
        depositor: &Keypair,
        depositor_token_account: &Pubkey,
        vault_token_account: &Pubkey,
        depositor_vrt_token_account: &Pubkey,
        vault_fee_token_account: &Pubkey,
        proof: &[[u8; 32]],
        amount_in: u64,
        min_amount_out: u64,
    ) -> TestResult<()> {
        let config = Config::find_program_address(&jito_vault_whitelist_program::id()).0;
        let signers = vec![depositor];
        let whitelist =
            Whitelist::find_program_address(&jito_vault_whitelist_program::id(), &vault_pubkey).0;

        let mut ix = MintBuilder::new()
            .config(config)
            .vault_config(
                jito_vault_core::config::Config::find_program_address(&jito_vault_program::id()).0,
            )
            .vault(*vault_pubkey)
            .vrt_mint(*vrt_mint)
            .depositor(depositor.pubkey())
            .depositor_token_account(*depositor_token_account)
            .vault_token_account(*vault_token_account)
            .depositor_vrt_token_account(*depositor_vrt_token_account)
            .vault_fee_token_account(*vault_fee_token_account)
            .whitelist(whitelist)
            .jito_vault_program(jito_vault_program::id())
            .token_program(spl_token::id())
            .proof(proof.to_vec())
            .amount_in(amount_in)
            .min_amount_out(min_amount_out)
            .instruction();
        ix.program_id = jito_vault_whitelist_program::id();

        let blockhash = self.banks_client.get_latest_blockhash().await?;

        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&depositor.pubkey()),
            &signers,
            blockhash,
        ))
        .await
    }
}

#[inline(always)]
#[track_caller]
pub fn assert_vault_whitelist_error<T>(
    test_error: Result<T, TestError>,
    vault_whitelist_error: VaultWhitelistError,
) {
    assert!(test_error.is_err());
    assert_eq!(
        test_error.err().unwrap().to_transaction_error().unwrap(),
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(vault_whitelist_error as u32)
        )
    );
}
