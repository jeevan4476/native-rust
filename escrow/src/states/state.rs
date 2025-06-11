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
    pub seed: u64,
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

        let bump = check_eq_PDA_and_get_bump(
            &[
                b"escrow",
                maker.as_ref(),
                escrow_data.seed.to_le_bytes().as_ref(),
            ],
            &crate::ID,
            escrow.key,
        )?;
        Ok((escrow_data, bump))
    }

    #[inline]
    pub fn init<'a>(
        seed: u64,
        receive: u64,
        mint_a: Pubkey,
        mint_b: Pubkey,
        maker: &AccountInfo<'a>,
        escrow: &AccountInfo<'a>,
    ) -> ProgramResult {
        let bump = check_eq_PDA_and_get_bump(
            &[b"escrow", maker.key.as_ref(), seed.to_le_bytes().as_ref()],
            &crate::ID,
            escrow.key,
        )?;
        let space = core::mem::size_of::<Escrow>();
        let rent = Rent::get()?.minimum_balance(space);

        let ix = create_account(maker.key, escrow.key, rent, space as u64, &crate::ID);
        let account_infos = vec![maker.clone(), escrow.clone()];
        //Create Escrow Account
        invoke_signed(
            &ix,
            &account_infos,
            &[&[
                b"escrow",
                maker.key.as_ref(),
                seed.to_le_bytes().as_ref(),
                &[bump],
            ]],
        )?;
        escrow.assign(&crate::ID);
        // Create the escrow
        let mut escrow_data: Escrow =
            *bytemuck::try_from_bytes_mut::<Escrow>(*escrow.data.borrow_mut())
                .map_err(|_| ProgramError::InvalidAccountData)?;
        escrow_data.clone_from(&Escrow {
            seed,
            maker: *maker.key,
            mint_a,
            mint_b,
            receive,
        });

        Ok(())
    }
}
