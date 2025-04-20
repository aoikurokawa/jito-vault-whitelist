use std::path::PathBuf;

use anyhow::anyhow;
use borsh::BorshDeserialize;
use jito_restaking_client_common::log::PrettyDisplay;
use jito_vault_whitelist_client::instructions::{
    InitializeConfigBuilder, InitializeWhitelistBuilder, MintBuilder, SetMintBurnAdminBuilder,
};
use log::{debug, info};
use meta_merkle_tree::{
    delegation::read_json_from_file, generated_merkle_tree::GeneratedMerkleTree,
    vault_whitelist_meta::VaultWhitelistMeta,
};
use solana_program::pubkey::Pubkey;
use solana_sdk::{signature::read_keypair_file, signer::Signer};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};

use crate::{
    cli_config::CliConfig,
    cli_signer::CliSigner,
    vault_whitelist::{ConfigActions, VaultWhitelistActions, VaultWhitelistCommands},
    CliHandler,
};

pub struct VaultWhitelistCliHandler {
    /// The configuration of CLI
    cli_config: CliConfig,

    /// The Pubkey of Jito Vault Whitelist Program ID
    vault_whitelist_program_id: Pubkey,

    /// The Pubkey of Jito Vault Program ID
    vault_program_id: Pubkey,

    /// This will print out the raw TX instead of running it
    print_tx: bool,
}

impl CliHandler for VaultWhitelistCliHandler {
    fn cli_config(&self) -> &CliConfig {
        &self.cli_config
    }

    fn print_tx(&self) -> bool {
        self.print_tx
    }
}

/// Handle Vault Whitelist
impl VaultWhitelistCliHandler {
    pub const fn new(
        cli_config: CliConfig,
        vault_whitelist_program_id: Pubkey,
        vault_program_id: Pubkey,
        print_tx: bool,
    ) -> Self {
        Self {
            cli_config,
            vault_whitelist_program_id,
            vault_program_id,
            print_tx,
        }
    }

    pub fn handle(&self, action: VaultWhitelistCommands) -> anyhow::Result<()> {
        match action {
            VaultWhitelistCommands::Config {
                action: ConfigActions::Initialize,
            } => self.initialize_config(),
            VaultWhitelistCommands::Config {
                action: ConfigActions::Get,
            } => self.get_config(),
            VaultWhitelistCommands::VaultWhitelist {
                action:
                    VaultWhitelistActions::Initialize {
                        whitelist_file_path,
                        vault,
                    },
            } => self.initialize_whitelist(whitelist_file_path, vault),
            VaultWhitelistCommands::VaultWhitelist {
                action: VaultWhitelistActions::SetMintBurnAdmin { vault },
            } => self.set_mint_burn_admin(vault),
            VaultWhitelistCommands::VaultWhitelist {
                action:
                    VaultWhitelistActions::Mint {
                        whitelist_file_path,
                        signer_keypair_path,
                        vault,
                        amount_in,
                        min_amount_out,
                    },
            } => self.mint(
                whitelist_file_path,
                signer_keypair_path,
                vault,
                amount_in,
                min_amount_out,
            ),
        }
    }
}

/// Handle Vault Whitelist Config
impl VaultWhitelistCliHandler {
    #[allow(clippy::future_not_send)]
    pub fn initialize_config(&self) -> anyhow::Result<()> {
        let signer = self.signer()?;

        let mut ix_builder = InitializeConfigBuilder::new();
        let config_address = jito_vault_whitelist_core::config::Config::find_program_address(
            &self.vault_whitelist_program_id,
        )
        .0;
        let ix_builder = ix_builder.config(config_address).admin(signer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.vault_whitelist_program_id;

        info!("Initializing vault config parameters: {:?}", ix_builder);

        self.process_transaction(&[ix], &signer.pubkey(), &[signer])?;

        if !self.print_tx {
            let account =
                self.get_account::<jito_vault_whitelist_client::accounts::Config>(&config_address)?;
            info!("{}", account.pretty_display());
        }

        Ok(())
    }

    #[allow(clippy::future_not_send)]
    fn get_config(&self) -> anyhow::Result<()> {
        let rpc_client = self.get_rpc_client();

        let config_address = jito_vault_whitelist_core::config::Config::find_program_address(
            &self.vault_whitelist_program_id,
        )
        .0;

        debug!(
            "Reading the jito vault whitelist configuration account at address: {}",
            config_address
        );

        let account = rpc_client.get_account(&config_address)?;
        let config = jito_vault_whitelist_client::accounts::Config::deserialize(
            &mut account.data.as_slice(),
        )?;
        info!("Vault config at address {}", config_address);
        info!("{}", config.pretty_display());
        Ok(())
    }
}

/// Handle Vault Whitelist Whitelist
impl VaultWhitelistCliHandler {
    #[allow(clippy::future_not_send)]
    pub fn initialize_whitelist(
        &self,
        whitelist_file_path: PathBuf,
        vault: Pubkey,
    ) -> anyhow::Result<()> {
        let signer = self.signer()?;
        let admin = signer.pubkey();

        let whitelist = jito_vault_whitelist_core::whitelist::Whitelist::find_program_address(
            &self.vault_whitelist_program_id,
            &vault,
        )
        .0;

        let vault_whitelist_metas =
            read_json_from_file::<Vec<VaultWhitelistMeta>>(&whitelist_file_path)?;
        let merkle_tree = GeneratedMerkleTree::new(&signer.pubkey(), &vault_whitelist_metas);

        let mut ix_builder = InitializeWhitelistBuilder::new();
        ix_builder
            .config(
                jito_vault_whitelist_core::config::Config::find_program_address(
                    &self.vault_whitelist_program_id,
                )
                .0,
            )
            .whitelist(whitelist)
            .vault(vault)
            .vault_admin(admin)
            .meta_merkle_root(merkle_tree.merkle_root.to_bytes());

        let mut ix = ix_builder.instruction();
        ix.program_id = self.vault_whitelist_program_id;

        info!("Initializing Whitelist at address: {}", whitelist);

        let ixs = [ix];
        self.process_transaction(&ixs, &signer.pubkey(), &[signer])?;

        if !self.print_tx {
            let account =
                self.get_account::<jito_vault_whitelist_client::accounts::Whitelist>(&whitelist)?;
            info!("{}", account.pretty_display());
        }

        Ok(())
    }

    pub fn set_mint_burn_admin(&self, vault: Pubkey) -> anyhow::Result<()> {
        let signer = self.signer()?;
        let admin = signer.pubkey();

        let whitelist = jito_vault_whitelist_core::whitelist::Whitelist::find_program_address(
            &self.vault_whitelist_program_id,
            &vault,
        )
        .0;

        let mut ix_builder = SetMintBurnAdminBuilder::new();
        ix_builder
            .config(
                jito_vault_whitelist_core::config::Config::find_program_address(
                    &self.vault_whitelist_program_id,
                )
                .0,
            )
            .vault_config(
                jito_vault_core::config::Config::find_program_address(&self.vault_program_id).0,
            )
            .whitelist(whitelist)
            .vault(vault)
            .vault_admin(admin)
            .jito_vault_program(self.vault_program_id);

        let mut ix = ix_builder.instruction();
        ix.program_id = self.vault_whitelist_program_id;

        info!("Setting Mint Burn Admin");

        let ixs = [ix];
        self.process_transaction(&ixs, &signer.pubkey(), &[signer])?;

        if !self.print_tx {
            let account =
                self.get_account::<jito_vault_whitelist_client::accounts::Whitelist>(&whitelist)?;
            info!("{}", account.pretty_display());
        }

        Ok(())
    }

    pub fn mint(
        &self,
        whitelist_file_path: PathBuf,
        signer_keypair_path: PathBuf,
        vault_pubkey: Pubkey,
        amount_in: u64,
        min_amount_out: u64,
    ) -> anyhow::Result<()> {
        let signer_keypair = read_keypair_file(signer_keypair_path)
            .map_err(|e| anyhow!("Failed to read signer keypair: {}", e))?;
        let signer = CliSigner::new(Some(signer_keypair), None);

        let whitelist = jito_vault_whitelist_core::whitelist::Whitelist::find_program_address(
            &self.vault_whitelist_program_id,
            &vault_pubkey,
        )
        .0;

        let vault = self.get_account::<jito_vault_client::accounts::Vault>(&vault_pubkey)?;

        let depositor = signer.pubkey();
        let depositor_token_account =
            get_associated_token_address(&depositor, &vault.supported_mint);
        let depositor_vrt_token_account = get_associated_token_address(&depositor, &vault.vrt_mint);

        let vault_token_account =
            get_associated_token_address(&vault_pubkey, &vault.supported_mint);

        let vault_fee_token_account =
            get_associated_token_address(&vault.fee_wallet, &vault.vrt_mint);

        let depositor_ata_ix = create_associated_token_account_idempotent(
            &depositor,
            &depositor,
            &vault.supported_mint,
            &spl_token::ID,
        );
        let depositor_vrt_ata_ix = create_associated_token_account_idempotent(
            &depositor,
            &depositor,
            &vault.vrt_mint,
            &spl_token::ID,
        );
        let vault_ata_ix = create_associated_token_account_idempotent(
            &depositor,
            &vault_pubkey,
            &vault.supported_mint,
            &spl_token::ID,
        );
        let vault_fee_ata_ix = create_associated_token_account_idempotent(
            &depositor,
            &vault.fee_wallet,
            &vault.vrt_mint,
            &spl_token::ID,
        );

        let vault_whitelist_metas =
            read_json_from_file::<Vec<VaultWhitelistMeta>>(&whitelist_file_path)?;
        let proof = GeneratedMerkleTree::get_proof(&vault_whitelist_metas, &depositor);

        let mut ix_builder = MintBuilder::new();
        ix_builder
            .config(
                jito_vault_whitelist_core::config::Config::find_program_address(
                    &self.vault_whitelist_program_id,
                )
                .0,
            )
            .vault_config(
                jito_vault_core::config::Config::find_program_address(&self.vault_program_id).0,
            )
            .vault(vault_pubkey)
            .vrt_mint(vault.vrt_mint)
            .depositor(depositor)
            .depositor_token_account(depositor_token_account)
            .vault_token_account(vault_token_account)
            .depositor_vrt_token_account(depositor_vrt_token_account)
            .vault_fee_token_account(vault_fee_token_account)
            .whitelist(whitelist)
            .jito_vault_program(self.vault_program_id)
            .proof(proof)
            .amount_in(amount_in)
            .min_amount_out(min_amount_out);

        let mut ix = ix_builder.instruction();
        ix.program_id = self.vault_whitelist_program_id;

        info!("Setting Mint Burn Admin");

        let ixs = [
            depositor_ata_ix,
            depositor_vrt_ata_ix,
            vault_ata_ix,
            vault_fee_ata_ix,
            ix,
        ];
        self.process_transaction(&ixs, &signer.pubkey(), &[signer])?;

        if !self.print_tx {
            let account =
                self.get_account::<jito_vault_whitelist_client::accounts::Whitelist>(&whitelist)?;
            info!("{}", account.pretty_display());
        }

        Ok(())
    }
}
