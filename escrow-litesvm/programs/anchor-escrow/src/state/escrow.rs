use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Escrow {
    pub offer_id: u64,
    pub owner: Pubkey,
    pub sell_token: Pubkey,
    pub buy_token: Pubkey,
    pub target_amount: u64,
    pub created_time: i64,
    pub vault_bump: u8,
}
