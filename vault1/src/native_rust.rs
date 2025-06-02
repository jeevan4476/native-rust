//program id 9zyF6XNYdJggdosddFYLGQMhsGWWbeP72BC6vps4yeM1

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum VaultInstruction {
    InitializeVault,
    DepositSol { amount: u64 },
    WithdrawSol { amount: u64 },
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VaultState {
    pub authority: Pubkey,
    pub bump_seed: u8,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VaultInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        VaultInstruction::InitializeVault => initialize_vault(program_id, accounts),
        VaultInstruction::DepositSol { amount } => deposit_sol(program_id, accounts, amount),
        VaultInstruction::WithdrawSol { amount } => withdraw_sol(program_id, accounts, amount),
    }
}

fn initialize_vault(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accs = &mut accounts.iter();
    let authority = next_account_info(accs)?; // signer
    let vault_pda = next_account_info(accs)?; // writable

    assert!(authority.is_signer);

    let (expected_pda, bump) =
        Pubkey::find_program_address(&[b"vault", authority.key.as_ref()], program_id);

    assert_eq!(*vault_pda.key, expected_pda);
    let mut vault_state = VaultState::try_from_slice(&vault_pda.data.borrow())?;

    vault_state.authority = *authority.key;
    vault_state.bump_seed = bump;

    vault_state.serialize(&mut &mut vault_pda.data.borrow_mut()[..])?;

    msg!("Vault initialized with authority: {}", authority.key);
    Ok(())
}

fn deposit_sol(_program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accs = &mut accounts.iter();
    let depositor = next_account_info(accs)?; // signer
    let vault_pda = next_account_info(accs)?; // writable
    let vault_authority = next_account_info(accs)?; // PDA authority for re-deriving check
    let system_program = next_account_info(accs)?;

    assert!(depositor.is_signer);

    let vault_state = VaultState::try_from_slice(&vault_pda.data.borrow())?;

    assert_eq!(*vault_authority.key, vault_state.authority);

    let insturction = system_instruction::transfer(depositor.key, vault_pda.key, amount);
    invoke(
        &insturction,
        &[depositor.clone(), vault_pda.clone(), system_program.clone()],
    )?;
    Ok(())
}

fn withdraw_sol(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accs = &mut accounts.iter();
    let authority = next_account_info(accs)?; // signer
    let vault_pda = next_account_info(accs)?; // writable

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_state = VaultState::try_from_slice(&vault_pda.data.borrow())?;

    assert_eq!(*authority.key, vault_state.authority);

    let (expected_pda, _) =
        Pubkey::find_program_address(&[b"vault", authority.key.as_ref()], program_id);

    assert_eq!(*vault_pda.key, expected_pda);
    if **vault_pda.lamports.borrow() < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    **vault_pda.try_borrow_mut_lamports()? -= amount;
    **authority.try_borrow_mut_lamports()? += amount;

    Ok(())
}
