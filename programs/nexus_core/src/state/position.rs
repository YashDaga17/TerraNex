use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Position {
    pub owner: Pubkey,
    pub pair: [u8; 16],
    pub base_position: i64,
    pub quote_position: i64,
    pub margin: u64,
    pub last_price: u64,
    pub bump: u8,
}
