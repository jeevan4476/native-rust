use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum VaultInstruction {
    InitialiseVault,
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
}
