use super::utils::{check_eq_PDA, check_eq_PDA_and_get_bump};
use bytemuck::{Pod, Zeroable};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};
use spl_token::instruction::{close_account, transfer_checked};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Escrow {
    pub send: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
}

impl Escrow {
    #[inline]
    pub fn get_data_and_bump(
        maker: &Pubkey,
        escrow: &AccountInfo,
    ) -> Result<(Escrow, u8), ProgramError> {
        //Get escrow data
        let escrow_data: Escrow = *bytemuck::try_from_bytes::<Escrow>(*escrow.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;

        //check PDA and return bump

        let bump = check_eq_PDA_and_get_bump(seeds, program_id, address)
    }
}
