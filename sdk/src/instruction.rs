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

    #[account(0, name = "config")]
    #[account(1, name = "vault")]
    #[account(2, writable, name = "whitelist")]
    #[account(3, signer, name = "vault_admin")]
    SetMetaMerkleRoot { meta_merkle_root: [u8; 32] },

    #[account(0, name = "config")]
    #[account(1, writable, name = "vault_config")]
    #[account(2, writable, name = "vault")]
    #[account(3, writable, name = "vrt_mint")]
    #[account(4, writable, signer, name = "depositor")]
    #[account(5, writable, name = "depositor_token_account")]
    #[account(6, writable, name = "vault_token_account")]
    #[account(7, writable, name = "depositor_vrt_token_account")]
    #[account(8, writable, name = "vault_fee_token_account")]
    #[account(9, writable, name = "whitelist")]
    #[account(10, name = "jito_vault_program")]
    #[account(11, name = "token_program")]
    Mint {
        proof: Vec<[u8; 32]>,
        amount_in: u64,
        min_amount_out: u64,
    },

    #[account(0, name = "config")]
    #[account(1, name = "vault_config")]
    #[account(2, name = "whitelist")]
    #[account(3, writable, name = "vault")]
    #[account(4, signer, name = "vault_admin")]
    #[account(5, name = "jito_vault_program")]
    CloseWhitelist,
}
