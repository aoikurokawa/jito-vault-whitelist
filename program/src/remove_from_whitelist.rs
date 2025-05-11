use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::{
    close_program_account,
    loader::{load_signer, load_system_program},
};
use jito_vault_core::vault::Vault;
use jito_vault_whitelist_core::{
    config::Config, whitelist::Whitelist, whitelist_user::WhitelistUser,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

/// Process removing user from whitelist
pub fn process_remove_from_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
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
    load_system_program(system_program_info)?;

    {
        WhitelistUser::load(
            program_id,
            whitelist_user_info,
            whitelist_info.key,
            user_info.key,
            true,
        )?;
        let whitelist_user_data = whitelist_user_info.data.borrow();
        let whitelist_user = WhitelistUser::try_from_slice_unchecked(&whitelist_user_data)?;

        whitelist_user.check_whitelist(whitelist_info.key)?;
        whitelist_user.check_user(user_info.key)?;
    }

    msg!(
        "Removing user {} from Whitelist {}",
        user_info.key,
        whitelist_info.key
    );

    close_program_account(program_id, whitelist_user_info, vault_admin_info)?;

    Ok(())
}
