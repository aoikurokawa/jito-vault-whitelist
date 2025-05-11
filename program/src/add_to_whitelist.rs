use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{create_account, loader::load_signer};
use jito_vault_core::vault::Vault;
use jito_vault_whitelist_core::{
    config::Config, whitelist::Whitelist, whitelist_user::WhitelistUser,
};
use jito_vault_whitelist_sdk::error::VaultWhitelistError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

/// Process adding new user to whitelist
pub fn process_add_to_whitelist(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [config_info, vault_info, whitelist_info, whitelist_user_info, vault_admin_info, user_info, system_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Config::load(program_id, config_info, false)?;

    Vault::load(&jito_vault_program::id(), vault_info, false)?;
    let vault_data = vault_info.data.borrow();
    let vault = Vault::try_from_slice_unchecked(&vault_data)?;

    vault.check_admin(vault_admin_info.key)?;

    {
        Whitelist::load(program_id, whitelist_info, vault_info.key, false)?;
        let whitelist_data = whitelist_info.data.borrow();
        let whitelist = Whitelist::try_from_slice_unchecked(&whitelist_data)?;

        whitelist.check_vault(vault_info.key)?;
    }

    load_signer(vault_admin_info, true)?;

    // The WhitelistUser account shall be at the canonical PDA
    let (whitelist_user_pubkey, whitelist_user_bump, mut whitelist_user_seeds) =
        WhitelistUser::find_program_address(program_id, whitelist_info.key, user_info.key);
    whitelist_user_seeds.push(vec![whitelist_user_bump]);
    if whitelist_user_pubkey.ne(whitelist_user_info.key) {
        msg!("WhitelistUser account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!(
        "Initializing WhitelistUser at address {}",
        whitelist_user_info.key
    );
    create_account(
        vault_admin_info,
        whitelist_user_info,
        system_program_info,
        program_id,
        &Rent::get()?,
        8_u64
            .checked_add(std::mem::size_of::<WhitelistUser>() as u64)
            .ok_or(VaultWhitelistError::ArithmeticOverflow)?,
        &whitelist_user_seeds,
    )?;

    let mut whitelist_user_data = whitelist_user_info.try_borrow_mut_data()?;
    whitelist_user_data[0] = WhitelistUser::DISCRIMINATOR;
    let whitelist_acc = WhitelistUser::try_from_slice_unchecked_mut(&mut whitelist_user_data)?;
    *whitelist_acc = WhitelistUser::new(*whitelist_info.key, *user_info.key, whitelist_user_bump);

    Ok(())
}
