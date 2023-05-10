use anchor_lang::{prelude::*, system_program};

use crate::{constants::DEPOSIT_PER_NFT, states::BankAccount};

#[derive(Accounts)]
pub struct DepositSolForNft<'info> {
    #[account(mut, has_one = bank_auth)]
    pub bank_account: Account<'info, BankAccount>,
    #[account(seeds = [b"pda-auth", bank_account.key().as_ref()], bump = bank_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol-vault", pda_auth.key().as_ref()], bump)]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub bank_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<DepositSolForNft>) -> Result<()> {
    let bank_account = &mut ctx.accounts.bank_account;

    // have to check if bank_auth has sufficient balance.

    let bank_auth = &ctx.accounts.bank_auth;
    let system_program = &ctx.accounts.system_program;

    bank_account.sol_vault_bump = ctx.bumps.get("sol_vault").copied();

    let cpi_accounts = system_program::Transfer {
        from: bank_auth.to_account_info(),
        to: ctx.accounts.sol_vault.to_account_info(),
    };

    let cpi = CpiContext::new(system_program.to_account_info(), cpi_accounts);

    system_program::transfer(cpi, DEPOSIT_PER_NFT)?;
    bank_account.nft_amount.increase_one();

    msg!("finished depositing sol for a NFT");
    Ok(())
}
