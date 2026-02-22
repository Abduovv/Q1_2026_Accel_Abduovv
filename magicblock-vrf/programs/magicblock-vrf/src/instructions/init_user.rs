use anchor_lang::prelude::*;

use crate::state::UserData;

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = UserData::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserData>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitUser<'info> {
    pub fn initialize(&mut self, bumps: &InitUserBumps) -> Result<()> {
        self.user_account.set_inner(UserData {
            owner: *self.user.key,
            stored_value: 0,
            signer_bump: bumps.user_account,
        });

        Ok(())
    }
}
