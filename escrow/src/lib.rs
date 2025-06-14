pub mod context;
mod instructions;
pub mod states;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey, pubkey::Pubkey,
};
#[cfg(test)]
mod tests;
use crate::{
    context::{make, refund, take},
    instructions::EscrowInstructions,
};

const ID: Pubkey = pubkey!("GYR4e4wWTg9KttwwjEsCmRPUsjxPzjEZ5BrhVFYm7KMW");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::ID) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match EscrowInstructions::try_from(discriminator)? {
        EscrowInstructions::Make => make::process(accounts, data),
        EscrowInstructions::Take => take::process(accounts),
        EscrowInstructions::Refund => refund::process(accounts),
    }
}
