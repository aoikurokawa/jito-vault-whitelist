use jito_vault_sdk::error::VaultError;
use solana_sdk::{instruction::InstructionError, transaction::TransactionError};

use crate::fixtures::TestError;

mod burn_withdrawal_ticket;
mod close_whitelist;
mod enqueue_withdrawal;
mod initialize_config;
mod initialize_whitelist;
mod mint;
mod set_meta_merkle_root;
mod set_mint_burn_admin;

#[inline(always)]
#[track_caller]
pub fn assert_vault_error<T>(test_error: Result<T, TestError>, vault_error: VaultError) {
    assert!(test_error.is_err());
    assert_eq!(
        test_error.err().unwrap().to_transaction_error().unwrap(),
        TransactionError::InstructionError(0, InstructionError::Custom(vault_error as u32))
    );
}
