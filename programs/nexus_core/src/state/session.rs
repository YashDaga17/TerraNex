use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TradingSession {
    pub authority: Pubkey,
    pub session_signer: Pubkey,
    pub expires_slot: u64,
    pub max_position_notional: u64,
    pub spent_notional: u64,
    pub allowed_pair: [u8; 16],
    pub active: bool,
    pub bump: u8,
}
