use add_to_whitelist::process_add_to_whitelist;
use borsh::BorshDeserialize;
use burn_withdrawal_ticket::process_burn_withdrawal_ticket;
use close_whitelist::process_close_whitelist;
use enqueue_withdrawal::process_enqueue_withdrawal;
use initialize_config::process_initialize_config;
use initialize_whitelist::process_initialize_whitelist;
use jito_vault_whitelist_sdk::instruction::VaultWhitelistInstruction;
use mint::process_mint;
use remove_from_whitelist::process_remove_from_whitelist;
use set_mint_burn_admin::process_set_mint_burn_admin;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

mod add_to_whitelist;
mod burn_withdrawal_ticket;
mod close_whitelist;
mod enqueue_withdrawal;
mod initialize_config;
mod initialize_whitelist;
mod mint;
mod remove_from_whitelist;
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

        VaultWhitelistInstruction::InitializeWhitelist => {
            msg!("Instruction: InitializeWhitelist");
            process_initialize_whitelist(program_id, accounts)
        }

        VaultWhitelistInstruction::SetMintBurnAdmin => {
            msg!("Instruction: SetMintBurnAdmin");
            process_set_mint_burn_admin(program_id, accounts)
        }

        VaultWhitelistInstruction::AddToWhitelist => {
            msg!("Instruction: AddToWhitelist");
            process_add_to_whitelist(program_id, accounts)
        }

        VaultWhitelistInstruction::RemoveFromWhitelist => {
            msg!("Instruction: RemoveFromWhitelist");
            process_remove_from_whitelist(program_id, accounts)
        }

        VaultWhitelistInstruction::Mint {
            amount_in,
            min_amount_out,
        } => {
            msg!("Instruction: Mint");
            process_mint(program_id, accounts, amount_in, min_amount_out)
        }

        VaultWhitelistInstruction::EnqueueWithdrawal { amount } => {
            msg!("Instruction: EnqueueWithdrawal");
            process_enqueue_withdrawal(program_id, accounts, amount)
        }

        VaultWhitelistInstruction::BurnWithdrawalTicket => {
            msg!("Instruction: BurnWithdrawalTicket");
            process_burn_withdrawal_ticket(program_id, accounts)
        }

        VaultWhitelistInstruction::CloseWhitelist => {
            msg!("Instruction: CloseWhitelist");
            process_close_whitelist(program_id, accounts)
        }
    }
}
