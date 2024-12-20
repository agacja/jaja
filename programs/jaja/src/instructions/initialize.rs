use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    associated_token::AssociatedToken,
};
use crate::{state::*, constants::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = ProgramState::LEN,
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, ProgramState>,

    /// CHECK: Retardio mint
    pub retardio_mint: UncheckedAccount<'info>,
    
    /// CHECK: PDA authority
    #[account(
        seeds = [AUTHORITY_SEED],
        bump
    )]
    pub authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<Initialize>, forward_address: Pubkey) -> Result<()> {
    let state = &mut ctx.accounts.state;
    let state_bump = ctx.bumps.state;
    let authority_bump = ctx.bumps.authority;
    
    state.initialize(
        ctx.accounts.owner.key(),
        forward_address,
        ctx.accounts.retardio_mint.key(),
        Pubkey::default(),
        ctx.accounts.authority.key(),
        Pubkey::default(),
        Pubkey::default(),
        Pubkey::default(),
        authority_bump,
        state_bump,
    )
}