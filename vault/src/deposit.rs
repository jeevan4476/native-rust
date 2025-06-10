use crate::state::VaultState;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::transfer,
};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let iter = &mut accounts.iter();
    let user = next_account_info(iter)?;
    let state_acc = next_account_info(iter)?;
    let vault_acc = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *state_acc.owner != *program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let _ = VaultState::try_from_slice(&state_acc.data.borrow())?;

    let (state_pda, _) =
        Pubkey::find_program_address(&[b"state".as_ref(), user.key.as_ref()], program_id);
    if state_pda != *state_acc.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let (vault_pda, _) =
        Pubkey::find_program_address(&[b"vault".as_ref(), state_acc.key.as_ref()], program_id);
    if vault_pda != *vault_acc.key {
        return Err(ProgramError::InvalidSeeds);
    }

    invoke(
        &transfer(user.key, vault_acc.key, amount),
        &[user.clone(), vault_acc.clone(), system_program.clone()],
    )
}
