use anchor_lang::prelude::*;

#[account]
pub struct AuthorizedUser {
    pub holder: Pubkey,
    pub balance: u64,
    pub signer_bump: u8,
}

impl AuthorizedUser {
    pub const LEN: usize = 8 + 32 + 8 + 1;
}
