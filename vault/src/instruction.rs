use solana_program::program_error::ProgramError;

pub enum VaultInstruction {
    InitialiseVault,
    Deposit,
    Withdraw,
    Close,
}

impl TryFrom<&u8> for VaultInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::InitialiseVault),
            1 => Ok(Self::Deposit),
            2 => Ok(Self::Withdraw),
            3 => Ok(Self::Close),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
