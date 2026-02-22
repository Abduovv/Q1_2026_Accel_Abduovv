use anchor_lang::prelude::*;

use crate::state::UserData;

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.signer_bump,
    )]
    pub user_account: Account<'info, UserData>,
}

impl<'info> UpdateUser<'info> {
    pub fn update(&mut self, new_value: u64) -> Result<()> {
        self.user_account.stored_value = new_value;

        Ok(())
    }
}
