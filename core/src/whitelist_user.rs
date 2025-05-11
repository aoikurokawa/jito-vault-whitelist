use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_vault_whitelist_sdk::error::VaultWhitelistError;
use shank::ShankAccount;
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct WhitelistUser {
    /// The base whitelist account that this user is derived from
    pub whitelist: Pubkey,

    /// The address of this user
    pub user: Pubkey,

    /// Bump seed for the PDA
    pub bump: u8,

    /// Reserved space
    reserved: [u8; 263],
}

impl WhitelistUser {
    pub const fn new(whitelist: Pubkey, user: Pubkey, bump: u8) -> Self {
        Self {
            whitelist,
            user,
            bump,
            reserved: [0; 263],
        }
    }

    /// Check whitelist pubkey
    pub fn check_whitelist(&self, whitelist: &Pubkey) -> Result<(), VaultWhitelistError> {
        if self.whitelist.ne(whitelist) {
            msg!("Whitelist pubkey does not match the provided whitelist pubkey");
            return Err(VaultWhitelistError::InvalidWhitelist);
        }

        Ok(())
    }

    /// Check user pubkey
    pub fn check_user(&self, user: &Pubkey) -> Result<(), VaultWhitelistError> {
        if self.user.ne(user) {
            msg!("User pubkey does not match the provided user pubkey");
            return Err(VaultWhitelistError::InvalidWhitelistUser);
        }

        Ok(())
    }

    /// Seeds of WhitelistUser Account
    pub fn seeds(whitelist: &Pubkey, user: &Pubkey) -> Vec<Vec<u8>> {
        vec![
            b"whitelist_user".to_vec(),
            whitelist.to_bytes().to_vec(),
            user.to_bytes().to_vec(),
        ]
    }

    /// Find the program address of WhitelistUser Account
    pub fn find_program_address(
        program_id: &Pubkey,
        whitelist: &Pubkey,
        user: &Pubkey,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(whitelist, user);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Load WhitelistUser Account
    pub fn load(
        program_id: &Pubkey,
        account: &AccountInfo,
        whitelist: &Pubkey,
        user: &Pubkey,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if account.owner.ne(program_id) {
            msg!("WhitelistUser account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("WhitelistUser account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable {
            msg!("WhitelistUser account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("WhitelistUser account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account
            .key
            .ne(&Self::find_program_address(program_id, whitelist, user).0)
        {
            msg!("WhitelistUser account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}
