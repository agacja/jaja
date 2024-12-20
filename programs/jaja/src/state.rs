use anchor_lang::prelude::*;
use crate::error::SwapError;

#[account]
#[derive(Default)]
pub struct ProgramState {
    pub owner: Pubkey,
    pub forward_address: Pubkey,
    pub retardio_mint: Pubkey,
    pub pool_id: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_sol_vault: Pubkey,
    pub pool_retardio_vault: Pubkey,
    pub program_token_account: Pubkey,
    pub authority_bump: u8,
    pub bump: u8,
    pub initialized: bool
}

impl ProgramState {
    pub const LEN: usize = 8 +  // Discriminator
        32 +                    // owner
        32 +                    // forward_address
        32 +                    // retardio_mint
        32 +                    // pool_id
        32 +                    // pool_authority
        32 +                    // pool_sol_vault
        32 +                    // pool_retardio_vault
        32 +                    // program_token_account
        1 +                     // authority_bump
        1 +                     // bump
        1;                      // initialized

    pub fn initialize(
        &mut self,
        owner: Pubkey,
        forward_address: Pubkey,
        retardio_mint: Pubkey,
        pool_id: Pubkey,
        pool_authority: Pubkey,
        pool_sol_vault: Pubkey,
        pool_retardio_vault: Pubkey,
        program_token_account: Pubkey,
        authority_bump: u8,
        bump: u8,
    ) -> Result<()> {
        require!(!self.initialized, SwapError::AlreadyInitialized);

        self.owner = owner;
        self.forward_address = forward_address;
        self.retardio_mint = retardio_mint;
        self.pool_id = pool_id;
        self.pool_authority = pool_authority;
        self.pool_sol_vault = pool_sol_vault;
        self.pool_retardio_vault = pool_retardio_vault;
        self.program_token_account = program_token_account;
        self.authority_bump = authority_bump;
        self.bump = bump;
        self.initialized = true;

        Ok(())
    }
}

#[event]
pub struct SwapEvent {
    pub user: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
    pub timestamp: i64,
    pub slippage: u64,
    pub pool_id: Pubkey,
}

#[event]
pub struct ForwardEvent {
    pub to_address: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub tx_signature: String,
}