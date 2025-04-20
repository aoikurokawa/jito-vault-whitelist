use borsh::BorshDeserialize;
use initialize_config::process_initialize_config;
use initialize_whitelist::process_initialize_whitelist;
use jito_vault_whitelist_sdk::instruction::VaultWhitelistInstruction;
use set_meta_merkle_root::process_set_meta_merkle_root;
use set_mint_burn_admin::process_set_mint_burn_admin;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

mod initialize_config;
mod initialize_whitelist;
mod mint;
mod set_meta_merkle_root;
mod set_mint_burn_admin;

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

        VaultWhitelistInstruction::InitializeWhitelist { meta_merkle_root } => {
            msg!("Instruction: InitializeWhitelist");
            process_initialize_whitelist(program_id, accounts, &meta_merkle_root)
        }

        VaultWhitelistInstruction::SetMintBurnAdmin => {
            msg!("Instruction: SetMintBurnAdmin");
            process_set_mint_burn_admin(program_id, accounts)
        }

        VaultWhitelistInstruction::SetMetaMerkleRoot { meta_merkle_root } => {
            msg!("Instruction: SetMetaMerkleRoot");
            process_set_meta_merkle_root(program_id, accounts, &meta_merkle_root)
        }
    }
}
