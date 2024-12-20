use anchor_lang::prelude::*;

#[error_code]
pub enum SwapError {
    #[msg("Program is already initialized")]
    AlreadyInitialized,
    #[msg("Amount must be greater than 0")]
    InvalidAmount,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Pool has insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Invalid forward address")]
    InvalidForwardAddress,
    #[msg("Swap timeout exceeded")]
    SwapTimeout,
}