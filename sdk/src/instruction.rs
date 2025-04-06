use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum VaultWhitelistInstruction {
    #[account(0, writable, name = "config")]
    #[account(1, name = "vault")]
    #[account(2, writable, signer, name = "vault_admin")]
    #[account(3, name = "system_program")]
    InitializeConfig,

    #[account(0, name = "config")]
    #[account(1, writable, name = "whitelist")]
    #[account(2, name = "vault")]
    #[account(3, writable, signer, name = "vault_admin")]
    #[account(4, name = "system_program")]
    InitializeWhitelist { meta_merkle_root: [u8; 32] },
}
