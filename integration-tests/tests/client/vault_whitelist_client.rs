use anchor_lang::AccountDeserialize;
use jito_vault_core::{
    config::Config as VaultConfig, vault::Vault,
    vault_staker_withdrawal_ticket::VaultStakerWithdrawalTicket,
};
use jito_vault_whitelist_client::instructions::{
    AddToWhitelistBuilder, BurnWithdrawalTicketBuilder, CloseWhitelistBuilder,
    EnqueueWithdrawalBuilder, InitializeConfigBuilder, InitializeWhitelistBuilder, MintBuilder,
    RemoveFromWhitelistBuilder, SetMintBurnAdminBuilder,
};
use jito_vault_whitelist_core::{
    config::Config, whitelist::Whitelist, whitelist_user::WhitelistUser,
};
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
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};

use crate::fixtures::{TestError, TestResult};

use super::vault_client::{VaultRoot, VaultStakerWithdrawalTicketRoot};

pub struct VaultWhitelistClient {
    pub banks_client: BanksClient,

    pub payer: Keypair,
}

impl VaultWhitelistClient {
    pub fn new(banks_client: BanksClient, payer: Keypair) -> Self {
        Self {
            banks_client,
            payer,
        }
    }

    pub async fn create_ata(&mut self, mint: &Pubkey, owner: &Pubkey) -> Result<(), TestError> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[create_associated_token_account_idempotent(
                        &self.payer.pubkey(),
                        owner,
                        mint,
                        &spl_token::id(),
                    )],
                    Some(&self.payer.pubkey()),
                    &[&self.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
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

    pub async fn get_config(
        &mut self,
    ) -> TestResult<jito_vault_whitelist_client::accounts::Config> {
        let config_pubkey = Config::find_program_address(&jito_vault_whitelist_program::id()).0;
        let account = self.banks_client.get_account(config_pubkey).await?.unwrap();
        let config = jito_vault_whitelist_client::accounts::Config::try_deserialize(
            &mut account.data.as_slice(),
        )
        .unwrap();

        Ok(config)
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

    pub async fn get_whitelist_user(
        &mut self,
        account: &Pubkey,
    ) -> TestResult<jito_vault_whitelist_client::accounts::WhitelistUser> {
        let account = self
            .banks_client
            .get_account(*account)
            .await?
            .ok_or(TestError::AccountNotFound)?;
        let whitelist = jito_vault_whitelist_client::accounts::WhitelistUser::try_deserialize(
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
            .jito_vault_program(jito_vault_program::id())
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

    pub async fn do_initialize_whitelist(&mut self, vault_root: &VaultRoot) -> TestResult<()> {
        self.initialize_whitelist(vault_root).await?;

        Ok(())
    }

    pub async fn initialize_whitelist(&mut self, vault_root: &VaultRoot) -> TestResult<()> {
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

    pub async fn do_add_to_whitelist(
        &mut self,
        vault_root: &VaultRoot,
        user: &Pubkey,
    ) -> TestResult<()> {
        self.add_to_whitelist(vault_root, user).await?;

        Ok(())
    }

    pub async fn add_to_whitelist(
        &mut self,
        vault_root: &VaultRoot,
        user: &Pubkey,
    ) -> TestResult<()> {
        let config = Config::find_program_address(&jito_vault_whitelist_program::id()).0;
        let whitelist = Whitelist::find_program_address(
            &jito_vault_whitelist_program::id(),
            &vault_root.vault_pubkey,
        )
        .0;
        let whitelist_user = WhitelistUser::find_program_address(
            &jito_vault_whitelist_program::id(),
            &whitelist,
            user,
        )
        .0;

        let mut ix = AddToWhitelistBuilder::new()
            .config(config)
            .whitelist(whitelist)
            .vault(vault_root.vault_pubkey)
            .vault_admin(vault_root.vault_admin.pubkey())
            .whitelist_user(whitelist_user)
            .user(*user)
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

    pub async fn do_remove_from_whitelist(
        &mut self,
        vault_root: &VaultRoot,
        user: &Pubkey,
    ) -> TestResult<()> {
        self.remove_from_whitelist(vault_root, user).await?;

        Ok(())
    }

    pub async fn remove_from_whitelist(
        &mut self,
        vault_root: &VaultRoot,
        user: &Pubkey,
    ) -> TestResult<()> {
        let config = Config::find_program_address(&jito_vault_whitelist_program::id()).0;
        let whitelist = Whitelist::find_program_address(
            &jito_vault_whitelist_program::id(),
            &vault_root.vault_pubkey,
        )
        .0;
        let whitelist_user = WhitelistUser::find_program_address(
            &jito_vault_whitelist_program::id(),
            &whitelist,
            user,
        )
        .0;

        let mut ix = RemoveFromWhitelistBuilder::new()
            .config(config)
            .whitelist(whitelist)
            .vault(vault_root.vault_pubkey)
            .vault_admin(vault_root.vault_admin.pubkey())
            .whitelist_user(whitelist_user)
            .user(*user)
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
        amount_in: u64,
        min_amount_out: u64,
    ) -> TestResult<()> {
        let config = Config::find_program_address(&jito_vault_whitelist_program::id()).0;
        let signers = vec![depositor];
        let whitelist =
            Whitelist::find_program_address(&jito_vault_whitelist_program::id(), &vault_pubkey).0;
        let whitelist_user = WhitelistUser::find_program_address(
            &jito_vault_whitelist_program::id(),
            &whitelist,
            &depositor.pubkey(),
        )
        .0;

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
            .whitelist_user(whitelist_user)
            .jito_vault_program(jito_vault_program::id())
            .token_program(spl_token::id())
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

    pub async fn do_enqueue_withdrawal(
        &mut self,
        vault_root: &VaultRoot,
        vault: &Vault,
        depositor: &Keypair,
        amount: u64,
    ) -> TestResult<VaultStakerWithdrawalTicketRoot> {
        let depositor_vrt_token_account =
            get_associated_token_address(&depositor.pubkey(), &vault.vrt_mint);

        let base = Keypair::new();
        let vault_staker_withdrawal_ticket = VaultStakerWithdrawalTicket::find_program_address(
            &jito_vault_program::id(),
            &vault_root.vault_pubkey,
            &base.pubkey(),
        )
        .0;
        let vault_staker_withdrawal_ticket_token_account =
            get_associated_token_address(&vault_staker_withdrawal_ticket, &vault.vrt_mint);

        self.create_ata(&vault.vrt_mint, &vault_staker_withdrawal_ticket)
            .await?;

        self.enqueue_withdrawal(
            &Config::find_program_address(&jito_vault_program::id()).0,
            &vault_root.vault_pubkey,
            &vault_staker_withdrawal_ticket,
            &vault_staker_withdrawal_ticket_token_account,
            depositor,
            &depositor_vrt_token_account,
            &base,
            amount,
        )
        .await?;

        Ok(VaultStakerWithdrawalTicketRoot {
            base: base.pubkey(),
        })
    }

    pub async fn enqueue_withdrawal(
        &mut self,
        _config: &Pubkey,
        vault: &Pubkey,
        vault_staker_withdrawal_ticket: &Pubkey,
        vault_staker_withdrawal_ticket_token_account: &Pubkey,
        staker: &Keypair,
        staker_vrt_token_account: &Pubkey,
        base: &Keypair,
        amount: u64,
    ) -> TestResult<()> {
        let config = Config::find_program_address(&jito_vault_whitelist_program::id()).0;
        let signers = vec![staker, base];
        let whitelist =
            Whitelist::find_program_address(&jito_vault_whitelist_program::id(), &vault).0;
        let whitelist_user = WhitelistUser::find_program_address(
            &jito_vault_whitelist_program::id(),
            &whitelist,
            &staker.pubkey(),
        )
        .0;

        let mut ix = EnqueueWithdrawalBuilder::new()
            .config(config)
            .vault_config(
                jito_vault_core::config::Config::find_program_address(&jito_vault_program::id()).0,
            )
            .vault(*vault)
            .vault_staker_withdrawal_ticket(*vault_staker_withdrawal_ticket)
            .vault_staker_withdrawal_ticket_token_account(
                *vault_staker_withdrawal_ticket_token_account,
            )
            .staker(staker.pubkey())
            .staker_vrt_token_account(*staker_vrt_token_account)
            .base(base.pubkey())
            .config(config)
            .whitelist(whitelist)
            .whitelist_user(whitelist_user)
            .jito_vault_program(jito_vault_program::id())
            .token_program(spl_token::id())
            .amount(amount)
            .instruction();
        ix.program_id = jito_vault_whitelist_program::id();

        let blockhash = self.banks_client.get_latest_blockhash().await?;

        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&staker.pubkey()),
            &signers,
            blockhash,
        ))
        .await
    }

    pub async fn do_burn_withdrawal_ticket(
        &mut self,
        config: &VaultConfig,
        vault_root: &VaultRoot,
        vault: &Vault,
        depositor: &Keypair,
        vault_staker_withdrawal_ticket_base: &Pubkey,
    ) -> TestResult<VaultStakerWithdrawalTicketRoot> {
        let base = Keypair::new();
        let vault_staker_withdrawal_ticket = VaultStakerWithdrawalTicket::find_program_address(
            &jito_vault_program::id(),
            &vault_root.vault_pubkey,
            vault_staker_withdrawal_ticket_base,
        )
        .0;
        let vault_token_account =
            get_associated_token_address(&vault_root.vault_pubkey, &vault.supported_mint);
        let vault_staker_withdrawal_ticket_token_account =
            get_associated_token_address(&vault_staker_withdrawal_ticket, &vault.vrt_mint);

        self.create_ata(&vault.vrt_mint, &vault_staker_withdrawal_ticket)
            .await?;

        self.burn_withdrawal_ticket(
            &Config::find_program_address(&jito_vault_program::id()).0,
            &vault_root.vault_pubkey,
            &vault_token_account,
            &vault.vrt_mint,
            depositor,
            &get_associated_token_address(&depositor.pubkey(), &vault.supported_mint),
            &vault_staker_withdrawal_ticket,
            &vault_staker_withdrawal_ticket_token_account,
            &get_associated_token_address(&vault.fee_wallet, &vault.vrt_mint),
            &get_associated_token_address(&config.program_fee_wallet, &vault.vrt_mint),
        )
        .await?;

        Ok(VaultStakerWithdrawalTicketRoot {
            base: base.pubkey(),
        })
    }

    pub async fn burn_withdrawal_ticket(
        &mut self,
        vault_config: &Pubkey,
        vault: &Pubkey,
        vault_token_account: &Pubkey,
        vrt_mint: &Pubkey,
        staker: &Keypair,
        staker_token_account: &Pubkey,
        vault_staker_withdrawal_ticket: &Pubkey,
        vault_staker_withdrawal_ticket_token_account: &Pubkey,
        vault_fee_token_account: &Pubkey,
        program_fee_token_account: &Pubkey,
    ) -> TestResult<()> {
        let signers = vec![staker];
        let whitelist =
            Whitelist::find_program_address(&jito_vault_whitelist_program::id(), &vault).0;
        let whitelist_user = WhitelistUser::find_program_address(
            &jito_vault_whitelist_program::id(),
            &whitelist,
            &staker.pubkey(),
        )
        .0;
        let config = Config::find_program_address(&jito_vault_whitelist_program::id()).0;

        let mut ix = BurnWithdrawalTicketBuilder::new()
            .vault_config(*vault_config)
            .vault(*vault)
            .vault_token_account(*vault_token_account)
            .vrt_mint(*vrt_mint)
            .staker(staker.pubkey())
            .staker_token_account(*staker_token_account)
            .vault_staker_withdrawal_ticket(*vault_staker_withdrawal_ticket)
            .vault_staker_withdrawal_ticket_token_account(
                *vault_staker_withdrawal_ticket_token_account,
            )
            .vault_fee_token_account(*vault_fee_token_account)
            .program_fee_token_account(*program_fee_token_account)
            .token_program(spl_token::id())
            .config(config)
            .whitelist(whitelist)
            .whitelist_user(whitelist_user)
            .jito_vault_program(jito_vault_program::id())
            .instruction();
        ix.program_id = jito_vault_whitelist_program::id();

        let blockhash = self.banks_client.get_latest_blockhash().await?;

        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&staker.pubkey()),
            &signers,
            blockhash,
        ))
        .await
    }

    pub async fn do_close_whitelist(&mut self, vault_root: &VaultRoot) -> TestResult<()> {
        self.close_whitelist(vault_root).await?;

        Ok(())
    }

    pub async fn close_whitelist(&mut self, vault_root: &VaultRoot) -> TestResult<()> {
        let config = Config::find_program_address(&jito_vault_whitelist_program::id()).0;
        let whitelist = Whitelist::find_program_address(
            &jito_vault_whitelist_program::id(),
            &vault_root.vault_pubkey,
        )
        .0;

        let mut ix = CloseWhitelistBuilder::new()
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
}

#[inline(always)]
#[track_caller]
#[allow(dead_code)]
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
