use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum VaultWhitelistInstruction {
    #[account(0, writable, name = "config")]
    #[account(1, name = "vault")]
    #[account(2, writable, signer, name = "vault_admin")]
    #[account(3, name = "system_program")]
    InitializeConfig,
}
