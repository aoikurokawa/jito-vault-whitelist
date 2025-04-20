use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_vault_core::vault::Vault;
use jito_vault_whitelist_core::{config::Config, whitelist::Whitelist};
use jito_vault_whitelist_sdk::error::VaultWhitelistError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

/// Process minting
pub fn process_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    proof: &[[u8; 32]],
    amount_in: u64,
    amount_out: u64,
) -> ProgramResult {
    let [config_info, vault_config, vault_info, vrt_mint, depositor, depositor_token_account, vault_token_account, depositor_vrt_token_account, vault_fee_token_account, whitelist_info, jito_vault_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Config::load(program_id, config_info, false)?;

    Vault::load(&jito_vault_program::id(), vault_info, false)?;
    Whitelist::load(program_id, whitelist_info, vault_info.key, true)?;
    let mut whitelist_data = whitelist_info.data.borrow_mut();
    let whitelist = Whitelist::try_from_slice_unchecked_mut(&mut whitelist_data)?;

    load_signer(depositor, false)?;

    // Verify the merkle proof.
    let node = &solana_program::hash::hashv(&[
        &[0u8],
        &solana_program::hash::hashv(&[&depositor.key.to_bytes(), &amount_in.to_le_bytes()])
            .to_bytes(),
    ]);

    if !meta_merkle_tree::verify::verify(
        proof.to_vec(),
        *whitelist.get_meta_merkle_root(),
        node.to_bytes(),
    ) {
        return Err(VaultWhitelistError::InvalidProof.into());
    }

    Ok(())
}
