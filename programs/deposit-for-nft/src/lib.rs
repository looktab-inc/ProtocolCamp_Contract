use anchor_lang::prelude::*;

pub mod errors;
pub mod constants;
mod instructions;
mod states;

pub use errors::DepositForNftError;
pub use instructions::initialize::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod deposit_for_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        
        instructions::initialize::handle(ctx)
    }
}

