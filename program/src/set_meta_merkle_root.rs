use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_vault_core::vault::Vault;
use jito_vault_whitelist_core::{config::Config, whitelist::Whitelist};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

/// Process setting meta merkle root for Whitelist
pub fn process_set_meta_merkle_root(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    meta_merkle_root: &[u8; 32],
) -> ProgramResult {
    let [config_info, vault_info, whitelist_info, vault_admin_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Config::load(program_id, config_info, false)?;

    Vault::load(&jito_vault_program::id(), vault_info, false)?;
    let vault_data = vault_info.data.borrow();
    let vault = Vault::try_from_slice_unchecked(&vault_data)?;

    vault.check_admin(vault_admin_info.key)?;

    Whitelist::load(program_id, whitelist_info, vault_info.key, true)?;
    let mut whitelist_data = whitelist_info.data.borrow_mut();
    let whitelist = Whitelist::try_from_slice_unchecked_mut(&mut whitelist_data)?;

    load_signer(vault_admin_info, true)?;

    whitelist.set_meta_merkle_root(meta_merkle_root);

    Ok(())
}
