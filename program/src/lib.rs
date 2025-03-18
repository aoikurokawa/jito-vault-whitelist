use borsh::BorshDeserialize;
use initialize_config::process_initialize_config;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};
use vault_whitelist_sdk::instruction::VaultWhitelistInstruction;

pub mod initialize_config;

declare_id!(env!("VAULT_WHITELIST_PROGRAM_ID"));

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = VaultWhitelistInstruction::try_from_slice(instruction_data)?;

    match instruction {
        VaultWhitelistInstruction::InitializeConfig => {
            msg!("Instruction: InitializeConfig");
            process_initialize_config(program_id, accounts)
        }
    }
}
