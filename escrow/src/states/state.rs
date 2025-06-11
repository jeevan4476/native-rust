use super::utils::{check_eq_pda, check_eq_pda_and_get_bump};
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

        let bump = check_eq_pda_and_get_bump(
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
        let bump = check_eq_pda_and_get_bump(
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

    #[inline]
    pub fn deposit<'a>(
        escrow_address: &Pubkey,
        token_program: &Pubkey,
        amount: u64,
        maker_ta_a: &AccountInfo<'a>,
        mint_a: &AccountInfo<'a>,
        vault: &AccountInfo<'a>,
        maker: &AccountInfo<'a>,
    ) -> ProgramResult {
        //check PDA of vault
        check_eq_pda_and_get_bump(&[b"vault", escrow_address.as_ref()], &crate::ID, vault.key)?;
        assert_eq!(
            *escrow_address,
            *<spl_token::state::Account as spl_token::state::GenericTokenAccount>::unpack_account_owner(*vault.try_borrow_data()?).ok_or(ProgramError::InvalidAccountData)?
        );

        let decimals = spl_token::state::Mint::unpack(&mint_a.try_borrow_data()?)?.decimals;
        let ix = transfer_checked(
            token_program,
            maker_ta_a.key,
            mint_a.key,
            vault.key,
            maker.key,
            &[],
            amount,
            decimals,
        )?;
        let account_info = vec![
            maker_ta_a.clone(),
            mint_a.clone(),
            vault.clone(),
            maker.clone(),
        ];
        invoke(&ix, &account_info)?;
        Ok(())
    }

    #[inline]
    pub fn take<'a>(
        escrow_data: Escrow,
        bump: u8,
        token_program: &Pubkey,
        mint_a: &AccountInfo<'a>,
        mint_b: &AccountInfo<'a>,
        vault: &AccountInfo<'a>,
        maker: &AccountInfo<'a>,
        taker: &AccountInfo<'a>,
        escrow: &AccountInfo<'a>,
        maker_ta_b: &AccountInfo<'a>,
        taker_ta_a: &AccountInfo<'a>,
        taker_ta_b: &AccountInfo<'a>,
    ) -> ProgramResult {
        check_eq_pda(&[b"vault", escrow.key.as_ref()], &crate::ID, vault.key)?;
        assert_eq!(mint_a.key, &escrow_data.mint_a);
        assert_eq!(mint_b.key, &escrow_data.mint_b);

        // Get token decimals
        let decimals_a = spl_token::state::Mint::unpack(&mint_a.try_borrow_data()?)?.decimals;
        let decimals_b = spl_token::state::Mint::unpack(&mint_b.try_borrow_data()?)?.decimals;

        // Get token amount
        let amount = spl_token::state::Account::unpack(&vault.try_borrow_data()?)?.amount;

        assert!([&spl_token::ID, &spl_token_2022::ID].contains(&token_program));

        //claim token A to taker
        let ix_1 = transfer_checked(
            token_program,
            vault.key,
            mint_a.key,
            taker_ta_a.key,
            escrow.key,
            &[],
            amount,
            decimals_a,
        )?;

        invoke_signed(
            &ix_1,
            &[
                vault.clone(),
                mint_a.clone(),
                taker_ta_a.clone(),
                escrow.clone(),
            ],
            &[&[
                b"escrow",
                maker.key.as_ref(),
                escrow_data.seed.to_le_bytes().as_ref(),
                &[bump],
            ]],
        )?;

        //transfer token B to maker

        invoke(
            &transfer_checked(
                token_program,
                taker_ta_b.key,
                mint_b.key,
                maker_ta_b.key,
                taker.key,
                &[],
                escrow_data.receive,
                decimals_b,
            )?,
            &[
                taker_ta_b.clone(),
                mint_b.clone(),
                maker_ta_b.clone(),
                taker.clone(),
            ],
        )?;

        //close the vault
        invoke_signed(
            &close_account(token_program, vault.key, maker.key, escrow.key, &[])?,
            &[vault.clone(), maker.clone(), escrow.clone()],
            &[&[
                b"escrow",
                maker.key.as_ref(),
                escrow_data.seed.to_le_bytes().as_ref(),
                &[bump],
            ]],
        )?;

        // Close the escrow
        let balance = escrow.lamports();
        escrow.realloc(0, false)?;
        **escrow.lamports.borrow_mut() = 0;
        **maker.lamports.borrow_mut() += balance;
        escrow.assign(&Pubkey::default());

        Ok(())
    }

    #[inline]
    pub fn refund<'a>(
        escrow_data: Escrow,
        bump: u8,
        token_program: &Pubkey,
        maker_ta_a: &AccountInfo<'a>,
        mint_a: &AccountInfo<'a>,
        escrow: &AccountInfo<'a>,
        vault: &AccountInfo<'a>,
        maker: &AccountInfo<'a>,
    ) -> ProgramResult {
        // Check PDA of vault
        check_eq_pda(&[b"vault", escrow.key.as_ref()], &crate::ID, vault.key)?;

        // Check mints match
        assert_eq!(mint_a.key, &escrow_data.mint_a);

        // Get token decimals
        let decimals = spl_token::state::Mint::unpack(&mint_a.try_borrow_data()?)?.decimals;

        // Get token amount
        let amount = spl_token::state::Account::unpack(&vault.try_borrow_data()?)?.amount;
        // By checking this, we know our token accounts are correct by virtue of Token Program checking them
        assert!([&spl_token::ID, &spl_token_2022::ID].contains(&token_program));

        invoke_signed(
            &transfer_checked(
                token_program,
                vault.key,
                mint_a.key,
                maker_ta_a.key,
                escrow.key,
                &[],
                amount,
                decimals,
            )?,
            &[
                vault.clone(),
                mint_a.clone(),
                maker_ta_a.clone(),
                escrow.clone(),
            ],
            &[&[
                b"escrow",
                maker.key.as_ref(),
                escrow_data.seed.to_le_bytes().as_ref(),
                &[bump],
            ]],
        )?;

        //close the vault
        invoke_signed(
            &close_account(token_program, vault.key, maker.key, escrow.key, &[])?,
            &[vault.clone(), maker.clone(), escrow.clone()],
            &[&[
                b"escrow",
                maker.key.as_ref(),
                escrow_data.seed.to_le_bytes().as_ref(),
                &[bump],
            ]],
        )?;

        // Close the escrow
        let balance = escrow.lamports();
        escrow.realloc(0, false)?;
        **escrow.lamports.borrow_mut() = 0;
        **maker.lamports.borrow_mut() += balance;
        escrow.assign(&Pubkey::default());

        Ok(())
    }
}
