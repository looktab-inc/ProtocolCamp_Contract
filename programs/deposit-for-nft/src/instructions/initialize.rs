use anchor_lang::prelude::*;

use crate::{states::BankAccount, errors::DepositForNftError};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = bank_auth, space = BankAccount::LEN)]
    pub bank_account: Account<'info, BankAccount>,
    /// CHECK: no need to check
    #[account(seeds = [b"auth", bank_account.key().as_ref()], bump)]
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub bank_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handle(ctx: Context<Initialize>) -> Result<()> {
    let bank_account = &mut ctx.accounts.bank_account;
    bank_account.deposit_auth = *ctx.accounts.bank_auth.key;
    if let Some(auth_bump) = ctx.bumps.get("pda_auth") {
        ctx.accounts.bank_account.auth_bump = *auth_bump;
        Ok(())
    } else {
        err!(DepositForNftError::InitializeError)
    }
}