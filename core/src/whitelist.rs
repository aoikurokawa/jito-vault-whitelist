use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use shank::ShankAccount;
use solana_program::msg;
use solana_program::pubkey::Pubkey;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct Whitelist {
    /// The vault pubkey
    vault: Pubkey,

    /// The merkle root of the meta merkle tree
    meta_merkle_root: [u8; 32],

    /// Bump seed for the PDA
    bump: u8,
}

impl Whitelist {
    /// Initiallize Whitelist
    pub const fn new(vault: Pubkey, meta_merkle_root: [u8; 32], bump: u8) -> Self {
        Self {
            vault,
            meta_merkle_root,
            bump,
        }
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
