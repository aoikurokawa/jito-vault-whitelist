use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_vault_whitelist_sdk::error::VaultWhitelistError;
use shank::ShankAccount;
use solana_program::msg;
use solana_program::pubkey::Pubkey;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

const RESERVED_SPACE_LEN: usize = 263;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct Whitelist {
    /// The vault pubkey
    pub vault: Pubkey,

    /// Bump seed for the PDA
    bump: u8,

    /// Reserved space
    reserved: [u8; 263],
}

impl Whitelist {
    /// Initiallize Whitelist
    pub const fn new(vault: Pubkey, bump: u8) -> Self {
        Self {
            vault,
            bump,
            reserved: [0; RESERVED_SPACE_LEN],
        }
    }

    /// Check Vault
    pub fn check_vault(&self, vault: &Pubkey) -> Result<(), VaultWhitelistError> {
        if self.vault.ne(vault) {
            msg!("Vault pubkey does not match the provided vault pubkey");
            return Err(VaultWhitelistError::InvalidVault);
        }

        Ok(())
    }

    /// Seeds of Whitelist Account
    pub fn seeds(vault: &Pubkey) -> Vec<Vec<u8>> {
        vec![b"whitelist".to_vec(), vault.to_bytes().to_vec()]
    }

    /// Find the program address of Whitelist Account
    pub fn find_program_address(program_id: &Pubkey, vault: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(vault);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Load Whitelist Account
    pub fn load(
        program_id: &Pubkey,
        account: &AccountInfo,
        vault: &Pubkey,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if account.owner.ne(program_id) {
            msg!("Whitelist account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("Whitelist account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable {
            msg!("Whitelist account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Whitelist account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account
            .key
            .ne(&Self::find_program_address(program_id, vault).0)
        {
            msg!("Whitelist account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitelist_no_padding() {
        let whitelist = std::mem::size_of::<Whitelist>();
        let sum_of_fields = size_of::<Pubkey>() + // vault
            size_of::<u8>() + // bump
            RESERVED_SPACE_LEN; // reserved
        assert_eq!(whitelist, sum_of_fields);
    }
}
