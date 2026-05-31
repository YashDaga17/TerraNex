use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::state::{ProtocolState, Vault};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeParams {
    pub solver: Pubkey,
    pub allowed_pair: [u8; 16],
    pub max_position_notional: u64,
}

#[derive(Accounts)]
#[instruction(params: InitializeParams)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ProtocolState::INIT_SPACE,
        seeds = [b"protocol"],
        bump
    )]
    pub protocol: Account<'info, ProtocolState>,
    #[account(
        init,
        payer = authority,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub lst_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol;
    protocol.authority = ctx.accounts.authority.key();
    protocol.solver = params.solver;
    protocol.vault = ctx.accounts.vault.key();
    protocol.allowed_pair = params.allowed_pair;
    protocol.max_position_notional = params.max_position_notional;
    protocol.bump = ctx.bumps.protocol;

    let vault = &mut ctx.accounts.vault;
    vault.authority = ctx.accounts.authority.key();
    vault.lst_mint = ctx.accounts.lst_mint.key();
    vault.total_lst_deposits = 0;
    vault.hedge_base_position = 0;
    vault.funding_collected = 0;
    vault.bump = ctx.bumps.vault;

    Ok(())
}
