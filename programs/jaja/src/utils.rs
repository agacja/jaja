use anchor_lang::prelude::*;
use crate::{error::SwapError, constants::*};

pub fn validate_pool_liquidity(
    input_vault: &AccountInfo,
    output_vault: &AccountInfo,
) -> Result<()> {
    require!(
        input_vault.lamports() >= MINIMUM_LIQUIDITY &&
        output_vault.lamports() >= MINIMUM_LIQUIDITY,
        SwapError::InsufficientLiquidity
    );
    Ok(())
}