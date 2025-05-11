use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_vault_sdk::sdk::enqueue_withdrawal;
use jito_vault_whitelist_core::{
    config::Config, whitelist::Whitelist, whitelist_user::WhitelistUser,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke_signed,
    program_error::ProgramError, pubkey::Pubkey,
};

/// Process enqueueing withdrawal
pub fn process_enqueue_withdrawal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    vrt_amount: u64,
) -> ProgramResult {
    let [vault_config_info, vault_info, vault_staker_withdrawal_ticket, vault_staker_withdrawal_ticket_token_account, staker, staker_vrt_token_account, base, token_program, system_program, config_info, whitelist_info, whitelist_user_info, jito_vault_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Config::load(program_id, config_info, false)?;

    Whitelist::load(program_id, whitelist_info, vault_info.key, true)?;
    let whitelist_data = whitelist_info.data.borrow();
    let whitelist = Whitelist::try_from_slice_unchecked(&whitelist_data)?;

    whitelist.check_vault(vault_info.key)?;

    load_signer(staker, true)?;

    WhitelistUser::load(
        program_id,
        whitelist_user_info,
        whitelist_info.key,
        staker.key,
        false,
    )?;
    let whitelist_user_data = whitelist_user_info.data.borrow();
    let whitelist_user = WhitelistUser::try_from_slice_unchecked(&whitelist_user_data)?;

    whitelist_user.check_whitelist(whitelist_info.key)?;
    whitelist_user.check_user(staker.key)?;

    let (_, whitelist_bump, mut whitelist_seeds) =
        Whitelist::find_program_address(program_id, vault_info.key);
    whitelist_seeds.push(vec![whitelist_bump]);

    let ix = enqueue_withdrawal(
        &jito_vault_program::id(),
        vault_config_info.key,
        vault_info.key,
        vault_staker_withdrawal_ticket.key,
        vault_staker_withdrawal_ticket_token_account.key,
        staker.key,
        staker_vrt_token_account.key,
        base.key,
        Some(whitelist_info.key),
        vrt_amount,
    );

    drop(whitelist_data);

    msg!("Processing enqueue_withdrawal instruction on Jito Vault Program");

    invoke_signed(
        &ix,
        &[
            vault_config_info.clone(),
            vault_info.clone(),
            vault_staker_withdrawal_ticket.clone(),
            vault_staker_withdrawal_ticket_token_account.clone(),
            staker.clone(),
            staker_vrt_token_account.clone(),
            base.clone(),
            whitelist_info.clone(),
            token_program.clone(),
            system_program.clone(),
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
