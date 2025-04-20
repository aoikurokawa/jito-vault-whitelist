use std::path::PathBuf;

use clap::{command, Subcommand};
use solana_sdk::pubkey::Pubkey;

#[derive(Subcommand)]
pub enum VaultWhitelistCommands {
    Config {
        #[command(subcommand)]
        action: ConfigActions,
    },
    VaultWhitelist {
        #[command(subcommand)]
        action: VaultWhitelistActions,
    },
}

#[derive(Subcommand)]
pub enum ConfigActions {
    /// Creates global config (can only be done once)
    Initialize,
    /// Fetches global config
    Get,
}

/// Vault Whitelist commands
#[derive(Subcommand)]
pub enum VaultWhitelistActions {
    /// Creates a new vault whitelist
    Initialize {
        whitelist_file_path: PathBuf,
        vault: Pubkey,
    },
}
