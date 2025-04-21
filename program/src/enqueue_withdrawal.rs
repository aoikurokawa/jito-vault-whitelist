use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_vault_core::vault::Vault;
use jito_vault_sdk::sdk::enqueue_withdrawal;
use jito_vault_whitelist_core::{config::Config, whitelist::Whitelist};
use jito_vault_whitelist_sdk::error::VaultWhitelistError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke_signed,
    program_error::ProgramError, pubkey::Pubkey,
};

/// Process enqueueing withdrawal
pub fn process_enqueue_withdrawal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    proof: &[[u8; 32]],
    vrt_amount: u64,
) -> ProgramResult {
    let [vault_config_info, vault_info, vault_staker_withdrawal_ticket, vault_staker_withdrawal_ticket_token_account, staker, staker_vrt_token_account, base, token_program, system_program, config_info, whitelist_info, jito_vault_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Config::load(program_id, config_info, false)?;

    Vault::load(&jito_vault_program::id(), vault_info, true)?;
    Whitelist::load(program_id, whitelist_info, vault_info.key, true)?;
    let whitelist_data = whitelist_info.data.borrow();
    let whitelist = Whitelist::try_from_slice_unchecked(&whitelist_data)?;

    load_signer(staker, false)?;

    // Verify the merkle proof.
    let node = &solana_program::hash::hashv(&[
        &[0u8],
        &solana_program::hash::hashv(&[&staker.key.to_bytes()]).to_bytes(),
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

    let ix = enqueue_withdrawal(
        &jito_vault_program::id(),
        vault_config_info.key,
        vault_info.key,
        vault_staker_withdrawal_ticket.key,
        vault_staker_withdrawal_ticket_token_account.key,
        staker.key,
        staker_vrt_token_account.key,
        base.key,
        vrt_amount,
    );

    drop(whitelist_data);

    msg!("Enqueue Withdrawal");

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
