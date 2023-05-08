use anchor_lang::{prelude::*};

use crate::{states::BankAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = bank_auth, space = BankAccount::LEN)]
    pub bank_account: Account<'info, BankAccount>,
    /// CHECK: no need to check
    #[account(seeds = [b"pda-auth", bank_account.key().as_ref()], bump)]
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub bank_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handle(ctx: Context<Initialize>) -> Result<()> {
    let bank_account = &mut ctx.accounts.bank_account;
    bank_account.deposit_auth = *ctx.accounts.bank_auth.key;
    bank_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();
    bank_account.nft_amount = 0;

    msg!("Smart Contract is initialized!");
    Ok(())
}