use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Invalid account configuration")]
    InvalidAccountConfig,
    #[msg("Mint setup failed")]
    MintSetupFailed,
    #[msg("Extension initialization error")]
    ExtensionInitError,
    #[msg("Access denied")]
    AccessDenied,
    #[msg("Extra account meta creation failed")]
    ExtraAccountMetaCreationFailed,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Math overflow detected")]
    MathOverflow,
}
