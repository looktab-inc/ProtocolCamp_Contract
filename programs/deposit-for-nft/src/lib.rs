use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
mod instructions;
mod states;

pub use errors::DepositForNftError;
pub use instructions::deposit::*;
pub use instructions::initialize::*;
pub use instructions::withdraw_for_burn::*;
pub use instructions::withdraw_for_expire::*;
pub use instructions::withdraw_for_verify::*;

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

    pub fn withdraw_for_burned(ctx: Context<WithdrawForBurn>) -> Result<()> {
        instructions::withdraw_for_burn::handle(ctx, 90, 10) // 99, 1
    }

    pub fn withdraw_for_expired(ctx: Context<WithdrawForExpire>) -> Result<()> {
        instructions::withdraw_for_expire::handle(ctx, 100, 0)
    }

    pub fn withdraw_for_verified(ctx: Context<WithdrawForVerify>) -> Result<()> {
        instructions::withdraw_for_verify::handle(ctx, 20, 80) // 20, 80
    }
}
