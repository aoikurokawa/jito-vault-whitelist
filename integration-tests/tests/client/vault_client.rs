use jito_bytemuck::AccountDeserialize;
use jito_vault_core::{burn_vault::BurnVault, config::Config, vault::Vault};
use jito_vault_sdk::sdk::{initialize_config, initialize_vault};
use solana_program::{
    native_token::sol_to_lamports,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{create_account, transfer},
};
use solana_program_test::{BanksClient, BanksClientError, ProgramTestBanksClientExt};
use solana_sdk::{
    commitment_config::CommitmentLevel, signature::Keypair, signer::Signer,
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};

use crate::fixtures::TestError;

pub struct VaultRoot {
    pub vault_pubkey: Pubkey,
    pub vault_admin: Keypair,
    pub mint: Keypair,
}

impl Clone for VaultRoot {
    fn clone(&self) -> Self {
        Self {
            vault_pubkey: self.vault_pubkey,
            vault_admin: self.vault_admin.insecure_clone(),
            mint: self.mint.insecure_clone(),
        }
    }
}

impl std::fmt::Debug for VaultRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VaultRoot {{ vault_pubkey: {}, vault_admin: {:?} }}",
            self.vault_pubkey, self.vault_admin
        )
    }
}

pub struct VaultProgramClient {
    banks_client: BanksClient,
    payer: Keypair,
}

impl VaultProgramClient {
    pub const fn new(banks_client: BanksClient, payer: Keypair) -> Self {
        Self {
            banks_client,
            payer,
        }
    }

    pub async fn airdrop(&mut self, to: &Pubkey, sol: f64) -> Result<(), TestError> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        let new_blockhash = self
            .banks_client
            .get_new_latest_blockhash(&blockhash)
            .await
            .unwrap();
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(&self.payer.pubkey(), to, sol_to_lamports(sol))],
                    Some(&self.payer.pubkey()),
                    &[&self.payer],
                    new_blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    async fn process_transaction(&mut self, tx: &Transaction) -> Result<(), TestError> {
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                tx.clone(),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    pub async fn create_token_mint(
        &mut self,
        mint: &Keypair,
        token_program_id: &Pubkey,
    ) -> Result<(), TestError> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        let rent: Rent = self.banks_client.get_sysvar().await?;
        let ixs = vec![
            create_account(
                &self.payer.pubkey(),
                &mint.pubkey(),
                rent.minimum_balance(spl_token::state::Mint::LEN),
                spl_token::state::Mint::LEN as u64,
                token_program_id,
            ),
            spl_token::instruction::initialize_mint2(
                token_program_id,
                &mint.pubkey(),
                &self.payer.pubkey(),
                None,
                9,
            )
            .unwrap(),
        ];
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &ixs,
                    Some(&self.payer.pubkey()),
                    &[&self.payer, mint],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
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

    pub async fn configure_depositor(
        &mut self,
        vault_root: &VaultRoot,
        depositor: &Pubkey,
        amount_to_mint: u64,
    ) -> Result<(), BanksClientError> {
        self.airdrop(depositor, 100.0).await.unwrap();
        let vault = self.get_vault(&vault_root.vault_pubkey).await.unwrap();
        self.create_ata(&vault.supported_mint, depositor)
            .await
            .unwrap();
        self.create_ata(&vault.vrt_mint, depositor).await.unwrap();
        self.create_ata(&vault.vrt_mint, &vault.fee_wallet)
            .await
            .unwrap();
        self.mint_spl_to(&vault.supported_mint, depositor, amount_to_mint)
            .await
            .unwrap();

        Ok(())
    }

    pub async fn mint_spl_to(
        &mut self,
        mint: &Pubkey,
        to: &Pubkey,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[
                        create_associated_token_account_idempotent(
                            &self.payer.pubkey(),
                            to,
                            mint,
                            &spl_token::id(),
                        ),
                        spl_token::instruction::mint_to(
                            &spl_token::id(),
                            mint,
                            &get_associated_token_address(to, mint),
                            &self.payer.pubkey(),
                            &[],
                            amount,
                        )
                        .unwrap(),
                    ],
                    Some(&self.payer.pubkey()),
                    &[&self.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await
    }

    #[allow(dead_code)]
    pub async fn get_config(&mut self, account: &Pubkey) -> Result<Config, TestError> {
        let account = self.banks_client.get_account(*account).await?.unwrap();
        Ok(*Config::try_from_slice_unchecked(account.data.as_slice())?)
    }

    #[allow(dead_code)]
    pub async fn get_vault(&mut self, account: &Pubkey) -> Result<Vault, TestError> {
        let account = self.banks_client.get_account(*account).await?.unwrap();
        Ok(*Vault::try_from_slice_unchecked(account.data.as_slice())?)
    }

    pub async fn do_initialize_config(&mut self) -> Result<Keypair, TestError> {
        let config_admin = Keypair::new();

        self.airdrop(&config_admin.pubkey(), 1.0).await?;

        let config_pubkey = Config::find_program_address(&jito_vault_program::id()).0;
        self.initialize_config(&config_pubkey, &config_admin, &config_admin.pubkey(), 0)
            .await?;

        Ok(config_admin)
    }

    pub async fn initialize_config(
        &mut self,
        config: &Pubkey,
        config_admin: &Keypair,
        program_fee_wallet: &Pubkey,
        program_fee_bps: u16,
    ) -> Result<(), TestError> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[initialize_config(
                &jito_vault_program::id(),
                config,
                &config_admin.pubkey(),
                &jito_restaking_program::id(),
                program_fee_wallet,
                program_fee_bps,
            )],
            Some(&config_admin.pubkey()),
            &[config_admin],
            blockhash,
        ))
        .await
    }

    pub async fn do_initialize_vault(
        &mut self,
        deposit_fee_bps: u16,
        withdrawal_fee_bps: u16,
        reward_fee_bps: u16,
        decimals: u8,
        program_fee_wallet: &Pubkey,
        token_mint: Option<Keypair>,
    ) -> Result<VaultRoot, TestError> {
        let vault_base = Keypair::new();

        let initialize_token_amount = Vault::DEFAULT_INITIALIZATION_TOKEN_AMOUNT;

        let vault_pubkey =
            Vault::find_program_address(&jito_vault_program::id(), &vault_base.pubkey()).0;

        let vrt_mint = Keypair::new();
        let vault_admin = Keypair::new();
        let token_mint = token_mint.unwrap_or_else(|| Keypair::new());

        self.airdrop(&vault_admin.pubkey(), 100.0).await?;

        let should_create_mint = {
            let raw_account = self.banks_client.get_account(token_mint.pubkey()).await?;
            raw_account.is_none()
        };
        if should_create_mint {
            self.create_token_mint(&token_mint, &spl_token::id())
                .await?;
        }

        self.initialize_vault(
            &Config::find_program_address(&jito_vault_program::id()).0,
            &vault_pubkey,
            &vrt_mint,
            &token_mint,
            &vault_admin,
            &vault_base,
            deposit_fee_bps,
            withdrawal_fee_bps,
            reward_fee_bps,
            decimals,
            initialize_token_amount,
        )
        .await?;

        // for holding the backed asset in the vault
        self.create_ata(&token_mint.pubkey(), &vault_pubkey).await?;
        // for holding fees
        self.create_ata(&vrt_mint.pubkey(), &vault_admin.pubkey())
            .await?;
        // for holding program fee
        self.create_ata(&vrt_mint.pubkey(), program_fee_wallet)
            .await?;

        // for holding program fee
        Ok(VaultRoot {
            vault_admin,
            vault_pubkey,
            mint: token_mint,
        })
    }

    pub async fn initialize_vault(
        &mut self,
        config: &Pubkey,
        vault: &Pubkey,
        vrt_mint: &Keypair,
        st_mint: &Keypair,
        vault_admin: &Keypair,
        vault_base: &Keypair,
        deposit_fee_bps: u16,
        withdrawal_fee_bps: u16,
        reward_fee_bps: u16,
        decimals: u8,
        initialize_token_amount: u64,
    ) -> Result<(), TestError> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;

        let admin_st_token_account =
            get_associated_token_address(&vault_admin.pubkey(), &st_mint.pubkey());
        let vault_st_token_account = get_associated_token_address(vault, &st_mint.pubkey());

        let burn_vault =
            BurnVault::find_program_address(&jito_vault_program::id(), &vault_base.pubkey()).0;

        let burn_vault_vrt_token_account =
            get_associated_token_address(&burn_vault, &vrt_mint.pubkey());

        self.create_ata(&st_mint.pubkey(), vault).await?;
        self.create_ata(&st_mint.pubkey(), &vault_admin.pubkey())
            .await?;

        self.mint_spl_to(
            &st_mint.pubkey(),
            &vault_admin.pubkey(),
            initialize_token_amount,
        )
        .await?;

        self.process_transaction(&Transaction::new_signed_with_payer(
            &[initialize_vault(
                &jito_vault_program::id(),
                config,
                vault,
                &vrt_mint.pubkey(),
                &st_mint.pubkey(),
                &admin_st_token_account,
                &vault_st_token_account,
                &burn_vault,
                &burn_vault_vrt_token_account,
                &vault_admin.pubkey(),
                &vault_base.pubkey(),
                deposit_fee_bps,
                withdrawal_fee_bps,
                reward_fee_bps,
                decimals,
                initialize_token_amount,
            )],
            Some(&vault_admin.pubkey()),
            &[&vault_admin, &vrt_mint, &vault_base],
            blockhash,
        ))
        .await
    }
}
