use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Default)]
pub struct VaultState {
    pub state_bump: u8,
    pub vault_bump: u8,
}

impl VaultState {
    pub const LEN: usize = 1 + 1;
}
