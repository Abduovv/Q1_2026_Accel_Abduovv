use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
    TransferChecked,
};

use crate::constants::ESCROW_SEED;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    sell_token: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = sell_token,
        associated_token::authority = maker,
    )]
    maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        has_one = sell_token,
        has_one = maker,
        seeds = [ESCROW_SEED, maker.key().as_ref(), escrow.offer_id.to_le_bytes().as_ref()],
        bump = escrow.vault_bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = sell_token,
        associated_token::authority = escrow,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn return_funds_and_close(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            ESCROW_SEED,
            self.escrow.owner.as_ref(),
            &self.escrow.offer_id.to_le_bytes()[..],
            &[self.escrow.vault_bump],
        ]];

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            mint: self.sell_token.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        transfer_checked(cpi_context, self.vault.amount, self.sell_token.decimals)?;

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        close_account(cpi_context)?;

        Ok(())
    }
}
