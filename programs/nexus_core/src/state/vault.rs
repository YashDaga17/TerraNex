use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProtocolState {
    pub authority: Pubkey,
    pub solver: Pubkey,
    pub vault: Pubkey,
    pub allowed_pair: [u8; 16],
    pub max_position_notional: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub authority: Pubkey,
    pub lst_mint: Pubkey,
    pub total_lst_deposits: u64,
    pub hedge_base_position: i64,
    pub funding_collected: i64,
    pub bump: u8,
}
