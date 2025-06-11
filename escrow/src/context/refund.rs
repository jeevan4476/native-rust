use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::states::Escrow;

pub fn process(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    let [maker, mint_a, maker_ta_a, escrow, vault, token_program, _system_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(maker.is_signer);
    let (escrow_data, bump) = Escrow::get_data_and_bump(maker.key, escrow)?;

    // Take: Claim token A to taker, Transfer token B to maker, Close the vault & escrow
    Escrow::refund(
        escrow_data,
        bump,
        token_program.key,
        maker_ta_a,
        mint_a,
        escrow,
        vault,
        maker,
    )
}
