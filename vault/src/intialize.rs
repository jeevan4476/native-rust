use crate::state::VaultState;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    // example_mocks::solana_sdk::system_instruction,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [user, state_acc, vault_acc, system_prorgam] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    // let accounts_iter = &mut accounts.iter();
    // let user = next_account_info(accounts_iter)?;
    // let state_acc = next_account_info(accounts_iter)?;
    // let vault_acc = next_account_info(accounts_iter)?;
    // let system_program = next_account_info(accounts_iter)?;

    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    };

    let (state_pda, state_bump) =
        Pubkey::find_program_address(&[b"state".as_ref(), user.key.as_ref()], program_id);
    if state_pda != *state_acc.key {
        return Err(ProgramError::InvalidSeeds);
    };
    let (vault_pda, vault_bump) =
        Pubkey::find_program_address(&[b"vault".as_ref(), state_acc.key.as_ref()], program_id);
    if vault_pda != *vault_acc.key {
        return Err(ProgramError::InvalidSeeds);
    };

    let rent = Rent::get()?;
    invoke(
        &create_account(
            user.key,
            state_acc.key,
            rent.minimum_balance(VaultState::LEN),
            VaultState::LEN as u64,
            program_id,
        ),
        &[user.clone(), state_acc.clone(), system_prorgam.clone()],
    )?;

    let vault_state = VaultState {
        state_bump,
        vault_bump,
    };
    vault_state.serialize(&mut *state_acc.data.borrow_mut())?;
    Ok(())
}
