use anchor_lang::prelude::*;

#[account]
pub struct UserData {
    pub owner: Pubkey,
    pub stored_value: u64,
    pub signer_bump: u8,
}

impl Space for UserData {
    const INIT_SPACE: usize = 32 + 8 + 1 + 8;
}
