use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_vault_sdk::sdk::mint_to;
use jito_vault_whitelist_core::{
    config::Config, whitelist::Whitelist, whitelist_user::WhitelistUser,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke_signed,
    program_error::ProgramError, pubkey::Pubkey,
};

/// Process minting
pub fn process_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount_in: u64,
    min_amount_out: u64,
) -> ProgramResult {
    let [config_info, vault_config_info, vault_info, vrt_mint, depositor, depositor_token_account, vault_token_account, depositor_vrt_token_account, vault_fee_token_account, whitelist_info, whitelist_user_info, jito_vault_program_info, token_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Config::load(program_id, config_info, false)?;

    Whitelist::load(program_id, whitelist_info, vault_info.key, true)?;
    let whitelist_data = whitelist_info.data.borrow();
    let whitelist = Whitelist::try_from_slice_unchecked(&whitelist_data)?;

    whitelist.check_vault(vault_info.key)?;

    load_signer(depositor, true)?;

    WhitelistUser::load(
        program_id,
        whitelist_user_info,
        whitelist_info.key,
        depositor.key,
        false,
    )?;
    let whitelist_user_data = whitelist_user_info.data.borrow();
    let whitelist_user = WhitelistUser::try_from_slice_unchecked(&whitelist_user_data)?;

    whitelist_user.check_whitelist(whitelist_info.key)?;
    whitelist_user.check_user(depositor.key)?;

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

    msg!("Processing mint_to instruction on Jito Vault Program");

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
