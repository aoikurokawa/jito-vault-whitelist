use solana_program::program_error::ProgramError;
use solana_program_test::BanksClientError;
use solana_sdk::{instruction::InstructionError, transaction::TransactionError};
use thiserror::Error;

pub mod fixture;

pub type TestResult<T> = Result<T, TestError>;

#[derive(Debug, Error)]
pub enum TestError {
    #[error(transparent)]
    BanksClient(#[from] BanksClientError),

    #[error(transparent)]
    Program(#[from] ProgramError),

    /// Account not found
    #[error("Account not found")]
    AccountNotFound,
}

impl TestError {
    pub fn to_transaction_error(&self) -> Option<TransactionError> {
        match self {
            TestError::BanksClient(e) => match e {
                BanksClientError::TransactionError(e) => Some(e.clone()),
                BanksClientError::SimulationError { err, .. } => Some(err.clone()),
                _ => None,
            },
            TestError::Program(_) => None,
            TestError::AccountNotFound => None,
        }
    }
}

#[inline(always)]
#[track_caller]
pub fn assert_ix_error<T>(test_error: Result<T, TestError>, ix_error: InstructionError) {
    assert!(test_error.is_err());
    assert_eq!(
        test_error.err().unwrap().to_transaction_error().unwrap(),
        TransactionError::InstructionError(0, ix_error)
    );
}
