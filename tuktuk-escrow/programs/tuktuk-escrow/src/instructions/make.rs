use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::constants::{ESCROW_SEED, TIME};
use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(offer_id: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,
    pub sell_mint: Account<'info, Mint>,
    pub buy_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = sell_mint,
        associated_token::authority = initiator,
    )]
    pub initiator_ata_sell: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initiator,
        seeds = [ESCROW_SEED, initiator.key().as_ref(), offer_id.to_le_bytes().as_ref()],
        bump,
        space = 8 + Escrow::INIT_SPACE,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = initiator,
        associated_token::mint = sell_mint,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, offer_id: u64, requested_amount: u64, bumps: &MakeBumps) -> Result<()> {
        let clock = Clock::get()?;
        let start_time = clock.unix_timestamp;

        self.escrow.set_inner(Escrow {
            offer_id,
            initiator: self.initiator.key(),
            sell_mint: self.sell_mint.key(),
            buy_mint: self.buy_mint.key(),
            requested_amount,
            start_time,
            expiry_time: start_time + TIME,
            vault_bump: bumps.escrow,
        });

        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.initiator_ata_sell.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.initiator.to_account_info(),
        };
        transfer(CpiContext::new(cpi_program, cpi_accounts), deposit)
    }
}
