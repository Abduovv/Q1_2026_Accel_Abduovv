use anchor_lang::{prelude::*, solana_program::instruction::Instruction};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use tuktuk_program::{
    compile_transaction,
    tuktuk::{
        cpi::{accounts::QueueTaskV0, queue_task_v0},
        program::Tuktuk,
        types::TriggerV0,
    },
    types::QueueTaskArgsV0,
    TransactionSourceV0,
};

use crate::constants::{AUTO_REFUND_DISCRIMINATOR, ESCROW_SEED, QUEUE_AUTHORITY_SEED};
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Schedule<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,
    pub sell_mint: Account<'info, Mint>,
    #[account(
        associated_token::mint = sell_mint,
        associated_token::authority = initiator,
    )]
    pub initiator_ata_sell: Account<'info, TokenAccount>,
    #[account(
        has_one = sell_mint,
        has_one = initiator,
        seeds = [ESCROW_SEED, initiator.key().as_ref(), escrow.offer_id.to_le_bytes().as_ref()],
        bump = escrow.vault_bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        associated_token::mint = sell_mint,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,

    /// CHECK: Passed through to TukTuk CPI
    #[account(mut)]
    pub task_queue: UncheckedAccount<'info>,

    /// CHECK: Derived and verified by TukTuk program
    #[account(mut)]
    pub task_queue_authority: UncheckedAccount<'info>,

    /// CHECK: Initialized in CPI - address = PDA(["task", task_queue, task_id], tuktuk)
    #[account(mut)]
    pub task: UncheckedAccount<'info>,

    /// CHECK: PDA signer - no data stored here
    #[account(
        mut,
        seeds = [QUEUE_AUTHORITY_SEED],
        bump,
    )]
    pub queue_authority: AccountInfo<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub tuktuk_program: Program<'info, Tuktuk>,
}

impl<'info> Schedule<'info> {
    pub fn schedule(&self, task_id: u16, bumps: &ScheduleBumps) -> Result<()> {
        let escrow_key = self.escrow.key();

        let auto_refund_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                anchor_lang::solana_program::instruction::AccountMeta::new(self.initiator.key(), false),
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.sell_mint.key(),
                    false,
                ),
                anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.initiator_ata_sell.key(),
                    false,
                ),
                anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.escrow.key(),
                    false,
                ),
                anchor_lang::solana_program::instruction::AccountMeta::new(self.vault.key(), false),
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program.key(),
                    false,
                ),
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.system_program.key(),
                    false,
                ),
            ],
            data: AUTO_REFUND_DISCRIMINATOR.to_vec(),
        };

        let (compiled_tx, _) = compile_transaction(vec![auto_refund_ix], vec![]).unwrap();

        queue_task_v0(
            CpiContext::new_with_signer(
                self.tuktuk_program.to_account_info(),
                QueueTaskV0 {
                    payer: self.initiator.to_account_info(),
                    queue_authority: self.queue_authority.to_account_info(),
                    task_queue: self.task_queue.to_account_info(),
                    task_queue_authority: self.task_queue_authority.to_account_info(),
                    task: self.task.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                },
                &[&[QUEUE_AUTHORITY_SEED, &[bumps.queue_authority]]],
            ),
            QueueTaskArgsV0 {
                id: task_id,
                trigger: TriggerV0::Timestamp(self.escrow.expiry_time),
                transaction: TransactionSourceV0::CompiledV0(compiled_tx),
                crank_reward: Some(5_000_000),
                free_tasks: 0,
                description: "escrow auto_refund on expiry".to_string(),
            },
        )?;

        msg!(
            "Scheduled auto_refund for escrow {} at timestamp {}",
            escrow_key,
            self.escrow.expiry_time
        );

        Ok(())
    }
}
