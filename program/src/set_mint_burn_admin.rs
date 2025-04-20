use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_vault_core::{config::Config as VaultConfig, vault::Vault};
use jito_vault_sdk::{instruction::VaultAdminRole, sdk::set_secondary_admin};
use jito_vault_whitelist_core::{config::Config, whitelist::Whitelist};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke,
    program_error::ProgramError, pubkey::Pubkey,
};

/// Process setting mint_burn_admin
pub fn process_set_mint_burn_admin(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [config_info, vault_config_info, whitelist_info, vault_info, vault_admin_info, jito_vault_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Config::load(program_id, config_info, false)?;
    VaultConfig::load(&jito_vault_program::id(), vault_config_info, false)?;
    Whitelist::load(program_id, whitelist_info, vault_info.key, false)?;

    Vault::load(&jito_vault_program::id(), vault_info, true)?;
    let vault_data = vault_info.data.borrow();
    let vault = Vault::try_from_slice_unchecked(&vault_data)?;

    vault.check_admin(vault_admin_info.key)?;

    load_signer(vault_admin_info, false)?;

    msg!("Setting MintBurnAdmin for Vault {}", vault_info.key);

    drop(vault_data);

    invoke(
        &set_secondary_admin(
            jito_vault_program_info.key,
            vault_config_info.key,
            vault_info.key,
            vault_admin_info.key,
            whitelist_info.key,
            VaultAdminRole::MintBurnAdmin,
        ),
        &[
            vault_config_info.clone(),
            vault_info.clone(),
            vault_admin_info.clone(),
            whitelist_info.clone(),
            jito_vault_program_info.clone(),
        ],
    )?;

    Ok(())
}
