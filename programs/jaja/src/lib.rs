use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use crate::instructions::initialize::Initialize;
use crate::instructions::swap::SwapAndForward;

declare_id!("6ar36VKC4QFyx3avJmBFg4C3121Yxu3mmTebbiJ3ASYU");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Jaja;

impl Jaja {
    pub fn initialize(
        ctx: Context<Initialize>, 
        forward_address: Pubkey
    ) -> Result<()> {
        instructions::initialize::handler(ctx, forward_address)
    }

    pub fn swap_and_forward(
        ctx: Context<SwapAndForward>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        instructions::swap::handler(ctx, amount_in, minimum_amount_out)
    }
}