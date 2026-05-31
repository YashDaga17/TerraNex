use anchor_lang::prelude::*;

use crate::error::NexusError;
use crate::state::{ProtocolState, TradingSession};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DelegateParams {
    pub session_signer: Pubkey,
    pub ttl_slots: u64,
    pub max_position_notional: u64,
    pub allowed_pair: [u8; 16],
}

#[derive(Accounts)]
#[instruction(params: DelegateParams)]
pub struct Delegate<'info> {
    #[account(seeds = [b"protocol"], bump = protocol.bump)]
    pub protocol: Account<'info, ProtocolState>,
    #[account(
        init,
        payer = authority,
        space = 8 + TradingSession::INIT_SPACE,
        seeds = [b"session", authority.key().as_ref(), params.session_signer.as_ref()],
        bump
    )]
    pub trading_session: Account<'info, TradingSession>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<Delegate>, params: DelegateParams) -> Result<()> {
    require!(
        params.allowed_pair == ctx.accounts.protocol.allowed_pair,
        NexusError::PairNotAllowed
    );
    require!(
        params.max_position_notional <= ctx.accounts.protocol.max_position_notional,
        NexusError::PositionCapExceeded
    );

    let clock = Clock::get()?;
    let session = &mut ctx.accounts.trading_session;
    session.authority = ctx.accounts.authority.key();
    session.session_signer = params.session_signer;
    session.expires_slot = clock
        .slot
        .checked_add(params.ttl_slots)
        .ok_or(NexusError::MathOverflow)?;
    session.max_position_notional = params.max_position_notional;
    session.spent_notional = 0;
    session.allowed_pair = params.allowed_pair;
    session.active = true;
    session.bump = ctx.bumps.trading_session;

    Ok(())
}
