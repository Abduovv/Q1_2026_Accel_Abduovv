use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::rnd::random_u64;

use crate::state::UserData;

#[derive(Accounts)]
pub struct ConsumeRandomnessCtx<'info> {
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserData>,
}

impl<'info> ConsumeRandomnessCtx<'info> {
    pub fn consume_randomness(&mut self, randomness: [u8; 32]) -> Result<()> {
        let rnd_value = random_u64(&randomness);
        self.user_account.stored_value = rnd_value;
        Ok(())
    }
}
