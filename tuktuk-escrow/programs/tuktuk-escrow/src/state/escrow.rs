use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Escrow {
    pub offer_id: u64,
    pub initiator: Pubkey,
    pub sell_mint: Pubkey,
    pub buy_mint: Pubkey,
    pub requested_amount: u64,
    pub start_time: i64,
    pub expiry_time: i64,
    pub vault_bump: u8,
}
