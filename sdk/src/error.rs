use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VaultWhitelistError {
    #[error("ArithmeticOverflow")]
    ArithmeticOverflow = 3000,

    #[error("ArithmeticUnderflow")]
    ArithmeticUnderflow,

    #[error("DivisionByZero")]
    DivisionByZero,

    #[error("InvalidProof")]
    InvalidProof,
}

impl From<VaultWhitelistError> for ProgramError {
    fn from(e: VaultWhitelistError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<VaultWhitelistError> for u64 {
    fn from(e: VaultWhitelistError) -> Self {
        e as Self
    }
}

impl From<VaultWhitelistError> for u32 {
    fn from(e: VaultWhitelistError) -> Self {
        e as Self
    }
}
