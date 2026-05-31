use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;
use state::{IntentParams, ResolveParams};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod nexus_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        initialize::handler(ctx, params)
    }

    pub fn delegate(ctx: Context<Delegate>, params: DelegateParams) -> Result<()> {
        delegate::handler(ctx, params)
    }

    pub fn submit_intent(ctx: Context<SubmitIntent>, params: IntentParams) -> Result<()> {
        submit_intent::handler(ctx, params)
    }

    pub fn resolve(ctx: Context<Resolve>, params: ResolveParams) -> Result<()> {
        resolve::handler(ctx, params)
    }
}
