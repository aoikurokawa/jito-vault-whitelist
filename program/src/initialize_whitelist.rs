use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use jito_vault_whitelist_core::{config::Config, whitelist::Whitelist};
use jito_vault_whitelist_sdk::error::VaultWhitelistError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

/// Process initializing Whitelist
pub fn process_initialize_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    meta_merkle_root: &[u8; 32],
) -> ProgramResult {
    let [config_info, whitelist_info, vault_info, vault_admin_info, system_program_info] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Config::load(program_id, config_info, vault_info.key, false)?;

    Whitelist::load(program_id, whitelist_info, vault_info.key, false)?;

    load_system_account(whitelist_info, true)?;
    load_signer(vault_admin_info, true)?;
    load_system_program(system_program_info)?;

    // The Whitelist account shall be at the canonical PDA
    let (whitelist_pubkey, whitelist_bump, mut whitelist_seeds) =
        Whitelist::find_program_address(program_id, vault_info.key);
    whitelist_seeds.push(vec![whitelist_bump]);
    if whitelist_pubkey.ne(whitelist_info.key) {
        msg!("Whitelist account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!(
        "Initializing Vault Whitelist at address {}",
        config_info.key
    );
    create_account(
        vault_admin_info,
        whitelist_info,
        system_program_info,
        program_id,
        &Rent::get()?,
        8_u64
            .checked_add(std::mem::size_of::<Config>() as u64)
            .ok_or(VaultWhitelistError::ArithmeticOverflow)?,
        &whitelist_seeds,
    )?;

    let mut whitelist_data = whitelist_info.try_borrow_mut_data()?;
    whitelist_data[0] = Whitelist::DISCRIMINATOR;
    let whitelist_acc = Whitelist::try_from_slice_unchecked_mut(&mut whitelist_data)?;
    *whitelist_acc = Whitelist::new(*vault_info.key, *meta_merkle_root, whitelist_bump);

    Ok(())
}
