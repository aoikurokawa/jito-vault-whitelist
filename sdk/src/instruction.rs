use borsh::{BorshDeserialize, BorshSerialize};
use codama::{codama, CodamaInstruction, CodamaInstructions};

#[derive(Debug, BorshSerialize, BorshDeserialize, CodamaInstructions)]
pub enum VaultWhitelistInstruction {
    #[codama(account(name = "config_info", writable))]
    #[codama(account(name = "vault_info"))]
    #[codama(account(name = "vault_admin_info", writable, signer))]
    #[codama(account(name = "system_program"))]
    InitializeConfig,
}
