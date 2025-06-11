use bytemuck::{Pod, Zeroable};

use solana_program::program_error::ProgramError;

pub enum EscrowInstructions {
    Make,
    Take,
    Refund,
}

impl TryFrom<&u8> for EscrowInstructions {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Make),
            1 => Ok(Self::Take),
            2 => Ok(Self::Take),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
