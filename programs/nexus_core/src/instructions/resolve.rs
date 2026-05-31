use anchor_lang::prelude::*;

use crate::error::NexusError;
use crate::state::{
    IntentState, Position, ProtocolState, ResolveParams, Vault, INTENT_OPEN, INTENT_RESOLVED,
    SIDE_LONG, SIDE_SHORT,
};

#[derive(Accounts)]
pub struct Resolve<'info> {
    #[account(seeds = [b"protocol"], bump = protocol.bump)]
    pub protocol: Account<'info, ProtocolState>,
    #[account(mut, seeds = [b"vault"], bump = vault.bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut, has_one = session)]
    pub intent: Account<'info, IntentState>,
    /// CHECK: Stored on the intent and used only as a PDA seed.
    pub session: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = solver,
        space = 8 + Position::INIT_SPACE,
        seeds = [b"position", intent.owner.as_ref(), intent.pair.as_ref()],
        bump
    )]
    pub position: Account<'info, Position>,
    #[account(mut)]
    pub solver: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<Resolve>, params: ResolveParams) -> Result<()> {
    let clock = Clock::get()?;
    let protocol = &ctx.accounts.protocol;
    let intent = &mut ctx.accounts.intent;

    require_keys_eq!(
        ctx.accounts.solver.key(),
        protocol.solver,
        NexusError::InvalidSolver
    );
    require!(
        intent.status == INTENT_OPEN,
        NexusError::IntentAlreadyResolved
    );
    require!(clock.slot <= intent.expires_slot, NexusError::IntentExpired);
    require!(params.filled_size > 0, NexusError::InvalidFill);
    require!(params.filled_size <= intent.size, NexusError::InvalidFill);
    require_price(intent, &params)?;

    let position = &mut ctx.accounts.position;
    if position.owner == Pubkey::default() {
        position.owner = intent.owner;
        position.pair = intent.pair;
        position.base_position = 0;
        position.quote_position = 0;
        position.margin = 0;
        position.bump = ctx.bumps.position;
    }

    let base_delta = params.filled_size as i64;
    let quote_delta = params
        .filled_size
        .checked_mul(params.execution_price)
        .ok_or(NexusError::MathOverflow)? as i64;

    match intent.side {
        SIDE_LONG => {
            position.base_position = position
                .base_position
                .checked_add(base_delta)
                .ok_or(NexusError::MathOverflow)?;
            position.quote_position = position
                .quote_position
                .checked_sub(quote_delta)
                .ok_or(NexusError::MathOverflow)?;
            ctx.accounts.vault.hedge_base_position = ctx
                .accounts
                .vault
                .hedge_base_position
                .checked_sub(base_delta)
                .ok_or(NexusError::MathOverflow)?;
        }
        SIDE_SHORT => {
            position.base_position = position
                .base_position
                .checked_sub(base_delta)
                .ok_or(NexusError::MathOverflow)?;
            position.quote_position = position
                .quote_position
                .checked_add(quote_delta)
                .ok_or(NexusError::MathOverflow)?;
            ctx.accounts.vault.hedge_base_position = ctx
                .accounts
                .vault
                .hedge_base_position
                .checked_add(base_delta)
                .ok_or(NexusError::MathOverflow)?;
        }
        _ => return err!(NexusError::InvalidSide),
    }

    position.last_price = params.execution_price;
    intent.status = INTENT_RESOLVED;
    intent.solver = ctx.accounts.solver.key();
    intent.execution_price = params.execution_price;
    intent.filled_size = params.filled_size;

    Ok(())
}

fn require_price(intent: &IntentState, params: &ResolveParams) -> Result<()> {
    match intent.side {
        SIDE_LONG => require!(
            params.execution_price <= intent.limit_price,
            NexusError::PriceOutOfBounds
        ),
        SIDE_SHORT => require!(
            params.execution_price >= intent.limit_price,
            NexusError::PriceOutOfBounds
        ),
        _ => return err!(NexusError::InvalidSide),
    }

    let band = params
        .oracle_price
        .checked_mul(intent.max_slippage_bps as u64)
        .ok_or(NexusError::MathOverflow)?
        .checked_div(10_000)
        .ok_or(NexusError::MathOverflow)?;
    let lower = params.oracle_price.saturating_sub(band);
    let upper = params
        .oracle_price
        .checked_add(band)
        .ok_or(NexusError::MathOverflow)?;

    require!(
        params.execution_price >= lower && params.execution_price <= upper,
        NexusError::PriceOutOfBounds
    );
    Ok(())
}
