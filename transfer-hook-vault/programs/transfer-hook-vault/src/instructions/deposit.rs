use anchor_lang::prelude::*;

use crate::{error::VaultError, AuthorizedUser, TokenVault, VAULT_CONFIG, WHITELIST_ENTRY};

#[derive(Accounts)]
pub struct Deposit<'info> {
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
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        self.user_account.balance = self
            .user_account
            .balance
            .checked_add(amount)
            .ok_or(VaultError::MathOverflow)?;

        msg!("Recorded deposit of {} tokens", amount);
        Ok(())
    }
}
