use anchor_lang::prelude::*;

#[error_code]
pub enum NexusError {
    #[msg("Session expired")]
    SessionExpired,
    #[msg("Session is not active")]
    SessionInactive,
    #[msg("Signer is not allowed for this session")]
    InvalidSessionSigner,
    #[msg("Intent asset pair is not allowed")]
    PairNotAllowed,
    #[msg("Intent exceeds the delegated notional cap")]
    PositionCapExceeded,
    #[msg("Intent has expired")]
    IntentExpired,
    #[msg("Intent has already been resolved")]
    IntentAlreadyResolved,
    #[msg("Execution price is outside the intent or oracle bounds")]
    PriceOutOfBounds,
    #[msg("Invalid trade side")]
    InvalidSide,
    #[msg("Invalid fill size")]
    InvalidFill,
    #[msg("Only the configured solver can resolve intents")]
    InvalidSolver,
    #[msg("Math overflow")]
    MathOverflow,
}
