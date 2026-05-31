use anchor_lang::prelude::*;

pub const SIDE_LONG: u8 = 0;
pub const SIDE_SHORT: u8 = 1;
pub const INTENT_OPEN: u8 = 0;
pub const INTENT_RESOLVED: u8 = 1;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct IntentParams {
    pub nonce: u64,
    pub pair: [u8; 16],
    pub side: u8,
    pub size: u64,
    pub limit_price: u64,
    pub max_slippage_bps: u16,
    pub expires_slot: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ResolveParams {
    pub execution_price: u64,
    pub oracle_price: u64,
    pub filled_size: u64,
}

#[account]
#[derive(InitSpace)]
pub struct IntentState {
    pub owner: Pubkey,
    pub session: Pubkey,
    pub nonce: u64,
    pub pair: [u8; 16],
    pub side: u8,
    pub size: u64,
    pub limit_price: u64,
    pub max_slippage_bps: u16,
    pub expires_slot: u64,
    pub status: u8,
    pub solver: Pubkey,
    pub execution_price: u64,
    pub filled_size: u64,
    pub bump: u8,
}
