use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_vault_core::vault::Vault;
use jito_vault_sdk::sdk::mint_to;
use jito_vault_whitelist_core::{config::Config, whitelist::Whitelist};
use jito_vault_whitelist_sdk::error::VaultWhitelistError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke_signed,
    program_error::ProgramError, pubkey::Pubkey,
};

/// Process minting
pub fn process_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    proof: &[[u8; 32]],
    amount_in: u64,
    min_amount_out: u64,
) -> ProgramResult {
    let [config_info, vault_config_info, vault_info, vrt_mint, depositor, depositor_token_account, vault_token_account, depositor_vrt_token_account, vault_fee_token_account, whitelist_info, jito_vault_program_info, token_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    msg!("Config Pubkey: {}", config_info.key);
    msg!("Vault Config Pubkey: {}", vault_config_info.key);
    msg!("Vault Pubkey: {}", vault_info.key);
    msg!("VRT Pubkey: {}", vrt_mint.key);
    msg!("Depositor Pubkey: {}", depositor.key);
    msg!("Whitelist Pubkey: {}", whitelist_info.key);

    Config::load(program_id, config_info, false)?;

    Vault::load(&jito_vault_program::id(), vault_info, true)?;
    Whitelist::load(program_id, whitelist_info, vault_info.key, true)?;
    let whitelist_data = whitelist_info.data.borrow();
    let whitelist = Whitelist::try_from_slice_unchecked(&whitelist_data)?;

    load_signer(depositor, true)?;

    // Verify the merkle proof.
    let node = &solana_program::hash::hashv(&[
        &[0u8],
        &solana_program::hash::hashv(&[&depositor.key.to_bytes()]).to_bytes(),
    ]);

    if !meta_merkle_tree::verify::verify(
        proof.to_vec(),
        *whitelist.get_meta_merkle_root(),
        node.to_bytes(),
    ) {
        return Err(VaultWhitelistError::InvalidProof.into());
    }

    let (_, whitelist_bump, mut whitelist_seeds) =
        Whitelist::find_program_address(program_id, vault_info.key);
    whitelist_seeds.push(vec![whitelist_bump]);

    let ix = mint_to(
        &jito_vault_program::id(),
        vault_config_info.key,
        vault_info.key,
        vrt_mint.key,
        depositor.key,
        depositor_token_account.key,
        vault_token_account.key,
        depositor_vrt_token_account.key,
        vault_fee_token_account.key,
        Some(whitelist_info.key),
        amount_in,
        min_amount_out,
    );

    drop(whitelist_data);

    invoke_signed(
        &ix,
        &[
            vault_config_info.clone(),
            vault_info.clone(),
            vrt_mint.clone(),
            depositor.clone(),
            depositor_token_account.clone(),
            vault_token_account.clone(),
            depositor_vrt_token_account.clone(),
            vault_fee_token_account.clone(),
            token_program_info.clone(),
            whitelist_info.clone(),
            jito_vault_program_info.clone(),
        ],
        &[whitelist_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()
            .as_slice()],
    )?;

    Ok(())
}
