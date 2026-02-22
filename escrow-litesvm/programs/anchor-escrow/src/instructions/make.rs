use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::constants::ESCROW_SEED;
use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(offer_id: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        seeds = [ESCROW_SEED, maker.key().as_ref(), offer_id.to_le_bytes().as_ref()],
        bump,
        space = 8 + Escrow::INIT_SPACE,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn setup_escrow(&mut self, offer_id: u64, target_amount: u64, bumps: &MakeBumps) -> Result<()> {
        let clock = Clock::get()?;
        self.escrow.set_inner(Escrow {
            offer_id,
            owner: self.maker.key(),
            sell_token: self.mint_a.key(),
            buy_token: self.mint_b.key(),
            target_amount,
            created_time: clock.unix_timestamp,
            vault_bump: bumps.escrow,
        });

        Ok(())
    }

    pub fn lock_funds(&mut self, lock_amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, lock_amount, self.mint_a.decimals)?;

        Ok(())
    }
}
