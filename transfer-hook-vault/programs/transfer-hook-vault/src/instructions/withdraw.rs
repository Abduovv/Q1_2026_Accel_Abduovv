use anchor_lang::prelude::*;
use anchor_spl::token_interface::{approve, Approve, TokenAccount, TokenInterface};

use crate::{error::VaultError, AuthorizedUser, TokenVault, VAULT_CONFIG, WHITELIST_ENTRY};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user: Signer<'info>,

    #[account(
        seeds = [VAULT_CONFIG.as_bytes(), vault.administrator.as_ref()],
        bump = vault.signer_bump,
    )]
    pub vault: Account<'info, TokenVault>,

    #[account(
        mut,
        seeds = [WHITELIST_ENTRY.as_bytes(), user.key().as_ref()],
        bump = user_account.signer_bump,
    )]
    pub user_account: Account<'info, AuthorizedUser>,

    #[account(
        mut,
        token::mint = vault.token_mint,
        token::authority = vault,
        token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(
            self.user_account.balance >= amount,
            VaultError::InsufficientBalance
        );

        let admin_key = self.vault.administrator;
        let bump = self.vault.signer_bump;
        let signer_seeds: &[&[&[u8]]] = &[&[VAULT_CONFIG.as_bytes(), admin_key.as_ref(), &[bump]]];

        approve(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Approve {
                    to: self.vault_token_account.to_account_info(),
                    delegate: self.user.to_account_info(),
                    authority: self.vault.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;

        self.user_account.balance = self
            .user_account
            .balance
            .checked_sub(amount)
            .ok_or(VaultError::MathOverflow)?;

        msg!("Approved withdrawal of {} tokens", amount);
        Ok(())
    }
}
