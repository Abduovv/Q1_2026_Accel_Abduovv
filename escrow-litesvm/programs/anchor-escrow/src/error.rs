use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Action not permitted at this time")]
    TimeLockActive,

    #[msg("Token account mismatch detected")]
    TokenAccountMismatch,

    #[msg("Account ownership validation failed")]
    OwnershipValidationFailed,
}
