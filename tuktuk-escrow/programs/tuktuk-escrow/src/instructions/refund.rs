use anchor_lang::prelude::*;
use anchor_spl::token::{
    close_account, transfer, CloseAccount, Mint, Token, TokenAccount, Transfer,
};

use crate::constants::ESCROW_SEED;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,
    pub sell_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = sell_mint,
        associated_token::authority = initiator,
    )]
    pub initiator_ata_sell: Account<'info, TokenAccount>,
    #[account(
        mut,
        close = initiator,
        has_one = sell_mint,
        has_one = initiator,
        seeds = [ESCROW_SEED, initiator.key().as_ref(), escrow.offer_id.to_le_bytes().as_ref()],
        bump = escrow.vault_bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = sell_mint,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn refund_and_close_vault(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            ESCROW_SEED,
            self.initiator.key.as_ref(),
            &self.escrow.offer_id.to_le_bytes()[..],
            &[self.escrow.vault_bump],
        ]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.initiator_ata_sell.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        transfer(
            CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds),
            self.vault.amount,
        )?;

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.initiator.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        close_account(CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            &signer_seeds,
        ))
    }
}
