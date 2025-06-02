use solana_program::entrypoint;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, hash::hashv, program_error::ProgramError,
    pubkey::Pubkey,
};
const ID: Pubkey = Pubkey::new_from_array([""]);

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

entrypoint!(process_instruction);

/// # Withdraw
///
/// Handles withdrawing funds from a PDA that has previously had lamports deposited to it.
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let [signer, vault] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(signer.is_signer);

    let lamports: u64 = u64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);
    let bump = data[8];
    let pda = hashv(&[signer.key.as_ref(), &[bump], ID.as_ref(), PDA_MARKER]);

    assert_eq!(pda.to_bytes(), vault.key.as_ref());

    **vault.try_borrow_mut_lamports()? -= lamports;
    **signer.try_borrow_mut_lamports()? += lamports;

    Ok(())
}
