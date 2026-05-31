use anchor_lang::prelude::*;
use session_keys::{session_auth_or, Session, SessionError, SessionTokenV2};

use crate::error::NexusError;
use crate::state::{IntentParams, IntentState, TradingSession, INTENT_OPEN, SIDE_LONG, SIDE_SHORT};

#[derive(Accounts, Session)]
#[instruction(params: IntentParams)]
pub struct SubmitIntent<'info> {
    #[account(
        mut,
        seeds = [
            b"session",
            trading_session.authority.as_ref(),
            trading_session.session_signer.as_ref()
        ],
        bump = trading_session.bump
    )]
    pub trading_session: Account<'info, TradingSession>,
    #[account(
        init,
        payer = signer,
        space = 8 + IntentState::INIT_SPACE,
        seeds = [b"intent", trading_session.authority.as_ref(), &params.nonce.to_le_bytes()],
        bump
    )]
    pub intent: Account<'info, IntentState>,
    #[session(
        signer = signer,
        authority = trading_session.authority.key()
    )]
    pub session_token: Option<Account<'info, SessionTokenV2>>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[session_auth_or(
    ctx.accounts.trading_session.authority == ctx.accounts.signer.key(),
    SessionError::InvalidToken
)]
pub(crate) fn handler(ctx: Context<SubmitIntent>, params: IntentParams) -> Result<()> {
    let clock = Clock::get()?;
    let session = &mut ctx.accounts.trading_session;

    require!(session.active, NexusError::SessionInactive);
    require!(
        clock.slot <= session.expires_slot,
        NexusError::SessionExpired
    );
    require!(clock.slot <= params.expires_slot, NexusError::IntentExpired);
    require!(
        params.expires_slot <= session.expires_slot,
        NexusError::SessionExpired
    );
    require!(
        params.pair == session.allowed_pair,
        NexusError::PairNotAllowed
    );
    require!(
        params.side == SIDE_LONG || params.side == SIDE_SHORT,
        NexusError::InvalidSide
    );

    let signed_by_owner = ctx.accounts.signer.key() == session.authority;
    let signed_by_session = ctx.accounts.signer.key() == session.session_signer;
    require!(
        signed_by_owner || signed_by_session,
        NexusError::InvalidSessionSigner
    );

    let notional = params
        .size
        .checked_mul(params.limit_price)
        .ok_or(NexusError::MathOverflow)?;
    let next_spent = session
        .spent_notional
        .checked_add(notional)
        .ok_or(NexusError::MathOverflow)?;
    require!(
        next_spent <= session.max_position_notional,
        NexusError::PositionCapExceeded
    );
    session.spent_notional = next_spent;

    let intent = &mut ctx.accounts.intent;
    intent.owner = session.authority;
    intent.session = session.key();
    intent.nonce = params.nonce;
    intent.pair = params.pair;
    intent.side = params.side;
    intent.size = params.size;
    intent.limit_price = params.limit_price;
    intent.max_slippage_bps = params.max_slippage_bps;
    intent.expires_slot = params.expires_slot;
    intent.status = INTENT_OPEN;
    intent.solver = Pubkey::default();
    intent.execution_price = 0;
    intent.filled_size = 0;
    intent.bump = ctx.bumps.intent;

    Ok(())
}
