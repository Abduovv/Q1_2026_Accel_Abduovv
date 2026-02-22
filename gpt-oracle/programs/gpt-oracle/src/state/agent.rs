use anchor_lang::prelude::*;

#[account]
pub struct OracleAgent {
    pub llm_context_addr: Pubkey,
    pub signer_bump: u8,
}
