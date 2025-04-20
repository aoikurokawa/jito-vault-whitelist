use std::{fs::File, path::PathBuf};

use anyhow::{anyhow, Result};
use borsh::BorshDeserialize;
use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::get_epoch;
use jito_vault_core::{
    burn_vault::BurnVault, config::Config, vault::Vault, vault_ncn_ticket::VaultNcnTicket,
    vault_operator_delegation::VaultOperatorDelegation,
    vault_staker_withdrawal_ticket::VaultStakerWithdrawalTicket,
    vault_update_state_tracker::VaultUpdateStateTracker,
};
use jito_vault_sdk::inline_mpl_token_metadata;
use jito_vault_whitelist_client::{
    instructions::{InitializeConfigBuilder, InitializeWhitelistBuilder},
    pretty_display::PrettyDisplay,
};
use log::{debug, info};
use meta_merkle_tree::{
    delegation::read_json_from_file, generated_merkle_tree::GeneratedMerkleTree,
    vault_whitelist_meta::VaultWhitelistMeta,
};
use solana_program::pubkey::Pubkey;
use solana_rpc_client::rpc_client::SerializableTransaction;
use solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};
use spl_token::instruction::transfer;

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

    #[allow(clippy::future_not_send)]
    pub async fn handle(&self, action: VaultWhitelistCommands) -> Result<()> {
        match action {
            VaultWhitelistCommands::Config {
                action: ConfigActions::Initialize,
            } => self.initialize_config().await,
            VaultWhitelistCommands::Config {
                action: ConfigActions::Get,
            } => self.get_config().await,
            VaultWhitelistCommands::VaultWhitelist {
                action:
                    VaultWhitelistActions::Initialize {
                        whitelist_file_path,
                        vault,
                    },
            } => self.initialize_whitelist(whitelist_file_path, vault).await,
        }
    }
}

/// Handle Vault Whitelist Config
impl VaultWhitelistCliHandler {
    #[allow(clippy::future_not_send)]
    pub async fn initialize_config(&self) -> Result<()> {
        let signer = self
            .cli_config
            .signer
            .as_ref()
            .ok_or_else(|| anyhow!("No Signer"))?;

        let mut ix_builder = InitializeConfigBuilder::new();
        let config_address = jito_vault_whitelist_core::config::Config::find_program_address(
            &self.vault_whitelist_program_id,
        )
        .0;
        let ix_builder = ix_builder.config(config_address).admin(signer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.vault_whitelist_program_id;

        info!("Initializing vault config parameters: {:?}", ix_builder);

        self.process_transaction(&[ix], &signer.pubkey(), &[signer])
            .await?;

        if !self.print_tx {
            let account = self
                .get_account::<jito_vault_whitelist_client::accounts::Config>(&config_address)
                .await?;
            info!("{}", account.pretty_display());
        }

        Ok(())
    }

    #[allow(clippy::future_not_send)]
    async fn get_config(&self) -> Result<()> {
        let rpc_client = self.get_rpc_client();

        let config_address = jito_vault_whitelist_core::config::Config::find_program_address(
            &self.vault_whitelist_program_id,
        )
        .0;

        debug!(
            "Reading the jito vault whitelist configuration account at address: {}",
            config_address
        );

        let account = rpc_client.get_account(&config_address).await?;
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
    #[allow(clippy::too_many_arguments, clippy::future_not_send)]
    pub async fn initialize_whitelist(
        &self,
        whitelist_file_path: PathBuf,
        vault: Pubkey,
    ) -> Result<()> {
        let signer = self
            .cli_config
            .signer
            .as_ref()
            .ok_or_else(|| anyhow!("No Signer"))?;

        let admin = signer.pubkey();

        let whitelist = jito_vault_whitelist_core::whitelist::Whitelist::find_program_address(
            &self.vault_whitelist_program_id,
            &vault,
        )
        .0;

        let vault_whitelist_metas =
            read_json_from_file::<Vec<VaultWhitelistMeta>>(&whitelist_file_path).unwrap();
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
        self.process_transaction(&ixs, &signer.pubkey(), &[signer])
            .await?;

        if !self.print_tx {
            let account = self
                .get_account::<jito_vault_whitelist_client::accounts::Whitelist>(&whitelist)
                .await?;
            info!("{}", account.pretty_display());
        }

        Ok(())
    }
}
