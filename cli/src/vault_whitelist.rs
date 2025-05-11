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
    Initialize { vault: Pubkey },

    /// Set mint burn admin
    SetMintBurnAdmin { vault: Pubkey },

    /// Add to whitelist
    AddToWhitelist { vault: Pubkey, user: Pubkey },

    /// Remove from whitelist
    RemoveFromWhitelist { vault: Pubkey, user: Pubkey },

    /// Mint
    Mint {
        signer_keypair_path: PathBuf,
        vault: Pubkey,
        amount_in: u64,
        min_amount_out: u64,
    },

    /// Enqueue Withdrawal
    EnqueueWithdrawal {
        signer_keypair_path: PathBuf,
        vault: Pubkey,
        amount: u64,
    },

    /// Burn Withdrawal Ticket
    BurnWithdrawalTicket {
        signer_keypair_path: PathBuf,
        vault: Pubkey,
    },

    /// Close whitelist
    CloseWhitelist { vault: Pubkey },
}
