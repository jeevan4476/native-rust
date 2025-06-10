use crate::state::VaultState;
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::transfer,
};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
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

    // Empty the vault PDA of all its lamports.
    let vault_lamports = **vault_account.lamports.borrow();
    if vault_lamports > 0 {
        let vault_seeds = &[
            b"vault".as_ref(),
            state_account.key.as_ref(),
            &[vault_state.vault_bump],
        ];
        let signer_seeds = &[&vault_seeds[..]];
        invoke_signed(
            &transfer(vault_account.key, user.key, vault_lamports),
            &[vault_account.clone(), user.clone(), system_program.clone()],
            signer_seeds,
        )?;
    }

    // Close the state account by transferring its rent back to the user.
    let state_lamports = **state_account.lamports.borrow();
    **user.lamports.borrow_mut() += state_lamports;
    **state_account.lamports.borrow_mut() = 0;
    state_account.data.borrow_mut().fill(0);

    Ok(())
}
