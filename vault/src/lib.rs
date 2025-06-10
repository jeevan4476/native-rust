pub mod instruction;

pub mod close;
pub mod deposit;
pub mod error;
pub mod intialize;
pub mod state;
pub mod withdraw;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey, pubkey::Pubkey,
};

use crate::instruction::VaultInstruction;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let instruction = VaultInstruction::try_from(discriminator)?;

    match instruction {
        VaultInstruction::InitialiseVault => {
            intialize::process(program_id, accounts)?;
        }
        VaultInstruction::Deposit => {
            let amount = data
                .get(..8)
                .and_then(|bytes| bytes.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            deposit::process(program_id, accounts, amount)?;
        }
        VaultInstruction::Withdraw => {
            let amount = data
                .get(..8)
                .and_then(|bytes| bytes.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            withdraw::process(program_id, accounts, amount)?;
        }
        VaultInstruction::Close => {
            close::process(program_id, accounts)?;
        }
    }
    Ok(())
}
