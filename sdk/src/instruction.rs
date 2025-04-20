use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum VaultWhitelistInstruction {
    #[account(0, writable, name = "config")]
    #[account(1, writable, signer, name = "admin")]
    #[account(2, name = "system_program")]
    InitializeConfig,

    #[account(0, name = "config")]
    #[account(1, writable, name = "whitelist")]
    #[account(2, name = "vault")]
    #[account(3, writable, signer, name = "vault_admin")]
    #[account(4, name = "system_program")]
    InitializeWhitelist { meta_merkle_root: [u8; 32] },

    #[account(0, name = "config")]
    #[account(1, name = "vault_config")]
    #[account(2, name = "whitelist")]
    #[account(3, writable, name = "vault")]
    #[account(4, signer, name = "vault_admin")]
    #[account(5, name = "jito_vault_program")]
    SetMintBurnAdmin,
}
