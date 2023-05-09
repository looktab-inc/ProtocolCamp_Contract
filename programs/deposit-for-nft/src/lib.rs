use anchor_lang::prelude::*;

pub mod errors;
pub mod constants;
mod instructions;
mod states;

pub use errors::DepositForNftError;
pub use instructions::initialize::*;
pub use instructions::deposit::*;
pub use instructions::withdraw::*;

declare_id!("GKcWYEKo8ZWKRo82e2Vgd92JwVeqwT5XNfyMmR4j4sfX");

#[program]
pub mod deposit_for_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handle(ctx)
    }

    pub fn deposit_for_nft(ctx: Context<DepositSolForNft>, amount: u64) -> Result<()> {
        instructions::deposit::handle(ctx, amount)
    }

    pub fn withdraw_half_for_nft(ctx: Context<WithdrawSolForNft>) -> Result<()> {
        instructions::withdraw::handle(ctx, 5.0, 5.0)
    }
}

