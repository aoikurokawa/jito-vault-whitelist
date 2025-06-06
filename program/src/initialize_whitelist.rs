use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use jito_vault_core::vault::Vault;
use jito_vault_whitelist_core::{config::Config, whitelist::Whitelist};
use jito_vault_whitelist_sdk::error::VaultWhitelistError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

/// Process initializing whitelist
pub fn process_initialize_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [config_info, whitelist_info, vault_info, vault_admin_info, system_program_info] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Config::load(program_id, config_info, false)?;

    load_system_account(whitelist_info, true)?;

    Vault::load(&jito_vault_program::id(), vault_info, false)?;
    let vault_data = vault_info.data.borrow();
    let vault = Vault::try_from_slice_unchecked(&vault_data)?;

    vault.check_admin(vault_admin_info.key)?;

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
        whitelist_info.key
    );
    create_account(
        vault_admin_info,
        whitelist_info,
        system_program_info,
        program_id,
        &Rent::get()?,
        8_u64
            .checked_add(std::mem::size_of::<Whitelist>() as u64)
            .ok_or(VaultWhitelistError::ArithmeticOverflow)?,
        &whitelist_seeds,
    )?;

    let mut whitelist_data = whitelist_info.try_borrow_mut_data()?;
    whitelist_data[0] = Whitelist::DISCRIMINATOR;
    let whitelist_acc = Whitelist::try_from_slice_unchecked_mut(&mut whitelist_data)?;
    *whitelist_acc = Whitelist::new(*vault_info.key, whitelist_bump);

    Ok(())
}
