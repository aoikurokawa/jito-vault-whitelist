use std::path::PathBuf;

use clap::{command, Subcommand};
use solana_sdk::pubkey::Pubkey;

#[derive(Subcommand)]
pub enum VaultWhitelistCommands {
    Config {
        #[command(subcommand)]
        action: ConfigActions,
    },
    Whitelist {
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

    /// Set mint burn admin
    SetMintBurnAdmin { vault: Pubkey },

    /// Set meta merkle root
    SetMetaMerkleRoot {
        whitelist_file_path: PathBuf,
        vault: Pubkey,
    },

    /// Mint
    Mint {
        whitelist_file_path: PathBuf,
        signer_keypair_path: PathBuf,
        vault: Pubkey,
        amount_in: u64,
        min_amount_out: u64,
    },

    /// Enqueue Withdrawal
    EnqueueWithdrawal {
        whitelist_file_path: PathBuf,
        signer_keypair_path: PathBuf,
        vault: Pubkey,
        amount: u64,
    },

    /// Close whitelist
    CloseWhitelist { vault: Pubkey },
}
