use crate::{error::VaultError, state::VaultState};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::transfer,
};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user = next_account_info(accounts_iter)?;
    let state_account = next_account_info(accounts_iter)?;
    let vault_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *state_account.owner != *program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let vault_state = VaultState::try_from_slice(&state_account.data.borrow())?;

    let (vault_pda, _) =
        Pubkey::find_program_address(&[b"vault".as_ref(), state_account.key.as_ref()], program_id);
    if vault_pda != *vault_account.key {
        return Err(ProgramError::InvalidSeeds);
    }

    if **vault_account.lamports.borrow() < amount {
        return Err(VaultError::InsufficientFunds.into());
    }

    let vault_seeds = &[
        b"vault".as_ref(),
        state_account.key.as_ref(),
        &[vault_state.vault_bump],
    ];
    let signer_seeds = &[&vault_seeds[..]];

    invoke_signed(
        &transfer(vault_account.key, user.key, amount),
        &[vault_account.clone(), user.clone(), system_program.clone()],
        signer_seeds,
    )?;

    Ok(())
}
