use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use shank::ShankAccount;
use solana_program::msg;
use solana_program::pubkey::Pubkey;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

const RESERVED_SPACE_LEN: usize = 263;

/// The vault whitelist configuration for the vault whitelist program
/// Manages program-wide settings and state.
#[derive(Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct Config {
    /// Config admin
    admin: Pubkey,

    /// The approved restaking program for this vault
    vault_program: Pubkey,

    /// The bump seed for the PDA
    pub bump: u8,

    /// Reserved space
    reserved: [u8; 263],
}

impl Config {
    /// Initialize Config
    pub const fn new(admin: Pubkey, vault_program: Pubkey, bump: u8) -> Self {
        Self {
            admin,
            vault_program,
            bump,
            reserved: [0; RESERVED_SPACE_LEN],
        }
    }

    /// Seeds of Config Account
    pub fn seeds() -> Vec<Vec<u8>> {
        vec![b"config".to_vec()]
    }

    /// Find the program address of Config Account
    pub fn find_program_address(program_id: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds();
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Load Config Account
    pub fn load(
        program_id: &Pubkey,
        account: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if account.owner.ne(program_id) {
            msg!("Config account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("Config account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable {
            msg!("Config account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Config account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.key.ne(&Self::find_program_address(program_id).0) {
            msg!("Config account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_no_padding() {
        let config = std::mem::size_of::<Config>();
        let sum_of_fields = size_of::<Pubkey>() + // admin
            size_of::<Pubkey>() + // vault_program
            size_of::<u8>() + // bump
            RESERVED_SPACE_LEN; // reserved
        assert_eq!(config, sum_of_fields);
    }
}
