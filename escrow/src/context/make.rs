use bytemuck::{Pod, Zeroable};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::states::Escrow;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
pub struct Make {
    pub seed: u64,
    pub amount: u64,
    pub recieve: u64,
}
impl TryFrom<&[u8]> for Make {
    type Error = ProgramError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        bytemuck::try_pod_read_unaligned::<Self>(value)
            .map_err(|_| ProgramError::InvalidInstructionData)
    }
}

//deposit funds into vault derived from Maker's pubkey and seeds
pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    let Make {
        seed,
        amount,
        recieve,
    } = Make::try_from(data)?;

    let [maker, mint_a, mint_b, maker_ta_a, escrow, vault, token_program, _system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //initialize escrow account and data

    //deposit funds to vault
    Ok(())
}
