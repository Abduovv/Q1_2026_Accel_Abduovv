use anchor_lang::prelude::*;

#[error_code]
pub enum OracleError {
    #[msg("Invalid account configuration")]
    InvalidAccountConfig,

    #[msg("Unauthorized access attempt")]
    UnauthorizedAccess,
}
