use std::{iter::FlatMap, ptr::null};

use mollusk_svm::{program, result::Check, Mollusk};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey,
    pubkey::Pubkey,
};

pub const PROGRAM: Pubkey = pubkey!("AS9D6BmDwdZuNDkgRCZxZaFK8yXSTgKBhTe22uwBsn1o");

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const user: Pubkey = pubkey!("2peQc98aHka64igrkSm9wXRiucEc9SeBcvY3KSG2w5Mr");
pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/vault");
    mollusk
}

#[test]
fn test_initialize() {
    let mollusk = mollusk();

    let (system_program, system_account) = program::keyed_account_for_system_program();

    let (state_pda, _bump) =
        Pubkey::find_program_address(&[b"state".as_ref(), &user.to_bytes()], &PROGRAM);

    let (vault_pda, _vault_bump) =
        Pubkey::find_program_address(&["vault".as_ref(), state_pda.as_ref()], &PROGRAM);

    let user_account = Account::new(1*LAMPORTS_PER_SOL, 0, &system_program);
    let state_account = Account::new(0, 0, &system_program);
    let vault_account = Account::new(0, 0, &system_program);

    let ix_accounts = vec![
        AccountMeta::new(user, true),
        AccountMeta::new(state_pda, false),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let ix = Instruction::new_with_bytes(program_id, ,ix_accounts);

    let tx_account = &vec![
        (user, user_account.clone()),
        (state_pda, state_account.clone()),
        (vault_pda, vault_account.clone()),
        (system_program, system_account.clone().into()),
    ];
    let _init_result = mollusk.process_and_validate_instruction(&i, accounts, checks)
}
