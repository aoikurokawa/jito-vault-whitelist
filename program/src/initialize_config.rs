use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};
use vault_whitelist_core::config::Config;
use vault_whitelist_sdk::error::VaultWhitelistError;

pub fn process_initialize_config(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [config_info, vault_info, vault_admin_info, system_program_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_system_account(config_info, true)?;
    load_signer(vault_admin_info, true)?;
    load_system_program(system_program_info)?;

    // The Config account shall be at the canonical PDA
    let (config_pubkey, config_bump, mut config_seeds) =
        Config::find_program_address(program_id, vault_info.key);
    config_seeds.push(vec![config_bump]);
    if config_pubkey.ne(config_info.key) {
        msg!("Config account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!(
        "Initializing Vault Whitelist at address {}",
        config_info.key
    );
    create_account(
        vault_admin_info,
        config_info,
        system_program_info,
        program_id,
        &Rent::get()?,
        8_u64
            .checked_add(std::mem::size_of::<Config>() as u64)
            .ok_or(VaultWhitelistError::ArithmeticOverflow)?,
        &config_seeds,
    )?;

    let mut config_data = config_info.try_borrow_mut_data()?;
    config_data[0] = Config::DISCRIMINATOR;
    let config_acc = Config::try_from_slice_unchecked_mut(&mut config_data)?;
    *config_acc = Config::new(*vault_info.key);

    Ok(())
}
