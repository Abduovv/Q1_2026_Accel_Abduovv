#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod constants;
mod error;
mod instructions;
mod state;
mod tests;

use instructions::*;

declare_id!("FircrADQ2wgGuvpm8qneNCfKM7o5zoHTWnDQxngpTQ3J");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, offer_id: u64, lock_amount: u64, target_amount: u64) -> Result<()> {
        ctx.accounts.setup_escrow(offer_id, target_amount, &ctx.bumps)?;
        ctx.accounts.lock_funds(lock_amount)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.return_funds_and_close()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.verify_conditions()?;
        ctx.accounts.transfer_payment()?;
        ctx.accounts.release_and_close()
    }
}
