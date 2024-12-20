use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use raydium_cp_swap::{
    cpi,
    program::RaydiumCpSwap,
    states::{AmmConfig, ObservationState, PoolState},
};
use crate::{state::*, error::*, constants::*};

#[derive(Accounts)]
pub struct SwapAndForward<'info> {
    pub cp_swap_program: Program<'info, RaydiumCpSwap>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        seeds = [STATE_SEED],
        bump = state.bump,
    )]
    pub state: Account<'info, ProgramState>,
    
    /// CHECK: Validated by seeds constraint
    #[account(
        seeds = [AUTHORITY_SEED],
        bump = state.authority_bump,
    )]
    pub authority: UncheckedAccount<'info>,

    #[account(address = pool_state.load()?.amm_config)]
    pub amm_config: Box<Account<'info, AmmConfig>>,
    
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    pub input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    
    #[account(mut)]
    pub output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    
    #[account(mut)]
    pub forward_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = input_vault.key() == pool_state.load()?.token_0_vault || 
                    input_vault.key() == pool_state.load()?.token_1_vault
    )]
    pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    
    #[account(
        mut,
        constraint = output_vault.key() == pool_state.load()?.token_0_vault || 
                    output_vault.key() == pool_state.load()?.token_1_vault
    )]
    pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        constraint = input_token_mint.key() == input_vault.mint
    )]
    pub input_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        constraint = output_token_mint.key() == output_vault.mint
    )]
    pub output_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        constraint = observation_state.key() == pool_state.load()?.observation_key
    )]
    pub observation_state: AccountLoader<'info, ObservationState>,

    pub input_token_program: Interface<'info, TokenInterface>,
    pub output_token_program: Interface<'info, TokenInterface>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<SwapAndForward>,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<()> {
    // Validate inputs
    require!(amount_in > 0, SwapError::InvalidAmount);
    require!(
        ctx.accounts.input_token_account.amount >= amount_in,
        SwapError::InsufficientBalance
    );

    // Verify pool liquidity
    let input_vault_info = ctx.accounts.input_vault.to_account_info();
    let output_vault_info = ctx.accounts.output_vault.to_account_info();

    require!(
        input_vault_info.lamports() >= MINIMUM_LIQUIDITY &&
        output_vault_info.lamports() >= MINIMUM_LIQUIDITY,
        SwapError::InsufficientLiquidity
    );

    // Perform swap
    let swap_accounts = cpi::accounts::Swap {
        payer: ctx.accounts.user.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        amm_config: ctx.accounts.amm_config.to_account_info(),
        pool_state: ctx.accounts.pool_state.to_account_info(),
        input_token_account: ctx.accounts.input_token_account.to_account_info(),
        output_token_account: ctx.accounts.output_token_account.to_account_info(),
        input_vault: ctx.accounts.input_vault.to_account_info(),
        output_vault: ctx.accounts.output_vault.to_account_info(),
        input_token_program: ctx.accounts.input_token_program.to_account_info(),
        output_token_program: ctx.accounts.output_token_program.to_account_info(),
        input_token_mint: ctx.accounts.input_token_mint.to_account_info(),
        output_token_mint: ctx.accounts.output_token_mint.to_account_info(),
        observation_state: ctx.accounts.observation_state.to_account_info(),
    };

    let swap_ctx = CpiContext::new(
        ctx.accounts.cp_swap_program.to_account_info(),
        swap_accounts,
    );

    raydium_cp_swap::cpi::swap_base_input(
        swap_ctx,
        amount_in,
        minimum_amount_out,
    )?;

    // Forward tokens
    let amount_to_forward = ctx.accounts.output_token_account.amount;
    
    let transfer_accounts = anchor_spl::token::Transfer {
        from: ctx.accounts.output_token_account.to_account_info(),
        to: ctx.accounts.forward_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    let authority_seeds = &[
        AUTHORITY_SEED,
        &[ctx.accounts.state.authority_bump]
    ];
    let authority_signer_seeds = &[&authority_seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        authority_signer_seeds,
    );

    anchor_spl::token::transfer(transfer_ctx, amount_to_forward)?;

    // Emit events
    emit!(SwapEvent {
        user: ctx.accounts.user.key(),
        amount_in,
        amount_out: amount_to_forward,
        timestamp: Clock::get()?.unix_timestamp,
        slippage: 0,
        pool_id: ctx.accounts.pool_state.key(),
    });

    emit!(ForwardEvent {
        to_address: ctx.accounts.forward_token_account.key(),
        amount: amount_to_forward,
        timestamp: Clock::get()?.unix_timestamp,
        tx_signature: "".to_string(),
    });
    
    Ok(())
}