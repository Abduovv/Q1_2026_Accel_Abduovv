use anchor_lang::prelude::*;
use anchor_spl::token::{
    close_account, transfer, CloseAccount, Mint, Token, TokenAccount, Transfer,
};

use crate::constants::ESCROW_SEED;
use crate::error::ErrorCode;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    /// CHECK: initiator just receives lamports
    #[account(mut)]
    pub initiator: AccountInfo<'info>,
    pub sell_mint: Account<'info, Mint>,
    pub buy_mint: Account<'info, Mint>,
    #[account(mut, token::mint = sell_mint, token::authority = taker)]
    pub taker_ata_sell: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::mint = buy_mint, token::authority = taker)]
    pub taker_ata_buy: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::mint = buy_mint, token::authority = initiator)]
    pub initiator_ata_buy: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        close = initiator,
        has_one = initiator,
        has_one = sell_mint,
        has_one = buy_mint,
        seeds = [ESCROW_SEED, initiator.key().as_ref(), escrow.offer_id.to_le_bytes().as_ref()],
        bump = escrow.vault_bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut, token::mint = sell_mint, token::authority = escrow)]
    pub vault: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn validate(&self) -> Result<()> {
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp < self.escrow.expiry_time,
            ErrorCode::EscrowExpired
        );
        Ok(())
    }

    pub fn deposit(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.taker_ata_buy.to_account_info(),
            to: self.initiator_ata_buy.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        transfer(
            CpiContext::new(cpi_program, cpi_accounts),
            self.escrow.requested_amount,
        )
    }

    pub fn withdraw_and_close_vault(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            ESCROW_SEED,
            self.initiator.key.as_ref(),
            &self.escrow.offer_id.to_le_bytes()[..],
            &[self.escrow.vault_bump],
        ]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.taker_ata_sell.to_account_info(),
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
