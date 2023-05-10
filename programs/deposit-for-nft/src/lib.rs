use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
mod instructions;
mod states;

pub use errors::DepositForNftError;
pub use instructions::deposit::*;
pub use instructions::initialize::*;
pub use instructions::withdraw::*;

declare_id!("GKcWYEKo8ZWKRo82e2Vgd92JwVeqwT5XNfyMmR4j4sfX");

#[program]
pub mod deposit_for_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handle(ctx)
    }

    pub fn deposit_for_nft(ctx: Context<DepositSolForNft>) -> Result<()> {
        instructions::deposit::handle(ctx)
    }

    pub fn withdraw_half_for_nft(ctx: Context<WithdrawSolForNft>) -> Result<()> {
        instructions::withdraw::handle(ctx, 5, 5)
    }

    pub fn withdraw_for_like(ctx: Context<WithdrawSolForNft>) -> Result<()> {
        // instructions::withdraw::handle(ctx, 9, 1)
        todo!()
    }

    pub fn withdraw_for_dislike(ctx: Context<WithdrawSolForNft>) -> Result<()> {
        // instructions::withdraw::handle(ctx, 1, 9)
        todo!()
    }
}
