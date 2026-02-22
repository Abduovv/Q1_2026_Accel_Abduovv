use anchor_lang::prelude::*;

#[account]
pub struct TokenVault {
    pub administrator: Pubkey,
    pub token_mint: Pubkey,
    pub signer_bump: u8,
}

impl TokenVault {
    pub const LEN: usize = 8 + 32 + 32 + 1;
}
