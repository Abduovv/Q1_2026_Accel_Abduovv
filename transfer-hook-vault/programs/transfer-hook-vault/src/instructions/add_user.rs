use anchor_lang::prelude::*;

use crate::{error::VaultError, AuthorizedUser, TokenVault, WHITELIST_ENTRY};

#[derive(Accounts)]
#[instruction(address: Pubkey)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        has_one = admin @ VaultError::AccessDenied,
    )]
    pub vault: Account<'info, TokenVault>,

    #[account(
        init,
        payer = admin,
        seeds = [WHITELIST_ENTRY.as_bytes(), address.as_ref()],
        bump,
        space = AuthorizedUser::LEN,
    )]
    pub user_account: Account<'info, AuthorizedUser>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddUser<'info> {
    pub fn add_user(&mut self, address: Pubkey, bump: &AddUserBumps) -> Result<()> {
        self.user_account.set_inner(AuthorizedUser {
            holder: address,
            balance: 0,
            signer_bump: bump.user_account,
        });

        msg!("Added {} to whitelist", address);

        Ok(())
    }
}
