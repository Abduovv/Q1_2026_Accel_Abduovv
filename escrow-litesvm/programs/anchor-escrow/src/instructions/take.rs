use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, transfer, CloseAccount, Mint, Token, TokenAccount, Transfer},
};

use crate::constants::{ESCROW_SEED, LOCK_PERIOD};
use crate::error::CustomError;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub owner: SystemAccount<'info>,
    pub sell_token: Account<'info, Mint>,
    pub buy_token: Account<'info, Mint>,
    #[account(mut)]
    pub taker_ata_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub taker_ata_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_ata_b: Account<'info, TokenAccount>,
    #[account(
        mut,
        close = owner,
        has_one = owner,
        has_one = sell_token,
        has_one = buy_token,
        seeds = [ESCROW_SEED, owner.key().as_ref(), escrow.offer_id.to_le_bytes().as_ref()],
        bump = escrow.vault_bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub clock: Sysvar<'info, Clock>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn verify_conditions(&self) -> Result<()> {
        let current_time = self.clock.unix_timestamp;

        require_keys_eq!(
            self.taker_ata_a.owner,
            self.taker.key(),
            CustomError::OwnershipValidationFailed
        );
        require_keys_eq!(
            self.taker_ata_a.mint,
            self.sell_token.key(),
            CustomError::TokenAccountMismatch
        );

        require_keys_eq!(
            self.taker_ata_b.owner,
            self.taker.key(),
            CustomError::OwnershipValidationFailed
        );
        require_keys_eq!(
            self.taker_ata_b.mint,
            self.buy_token.key(),
            CustomError::TokenAccountMismatch
        );

        require_keys_eq!(
            self.owner_ata_b.owner,
            self.owner.key(),
            CustomError::OwnershipValidationFailed
        );
        require_keys_eq!(
            self.owner_ata_b.mint,
            self.buy_token.key(),
            CustomError::TokenAccountMismatch
        );

        require_keys_eq!(
            self.vault.owner,
            self.escrow.key(),
            CustomError::OwnershipValidationFailed
        );
        require_keys_eq!(
            self.vault.mint,
            self.sell_token.key(),
            CustomError::TokenAccountMismatch
        );

        require!(
            current_time >= self.escrow.created_time + LOCK_PERIOD,
            CustomError::TimeLockActive,
        );

        Ok(())
    }

    pub fn transfer_payment(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.taker_ata_b.to_account_info(),
            to: self.owner_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, self.escrow.target_amount)
    }

    pub fn release_and_close(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            ESCROW_SEED,
            self.escrow.owner.as_ref(),
            &self.escrow.offer_id.to_le_bytes()[..],
            &[self.escrow.vault_bump],
        ]];

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        transfer(cpi_context, self.vault.amount)?;

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.owner.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        close_account(cpi_context)
    }
}
