use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program};

use crate::{states::BankAccount, DepositForNftError};

#[derive(Accounts)]
pub struct WithdrawSolForNft<'info> {
    #[account(has_one = bank_auth)]
    pub bank_account: Account<'info, BankAccount>,
    #[account(seeds = [b"pda-auth", bank_account.key().as_ref()], bump = bank_account.auth_bump)]
    /// CHECK: no need to check this
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol-vault", pda_auth.key().as_ref()], bump = bank_account.sol_vault_bump.unwrap())]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub bank_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub client_account: SystemAccount<'info>,
}

pub fn handle(ctx: Context<WithdrawSolForNft>, client_ratio: u8, bank_ratio: u8) -> Result<()> {
    msg!("withdraw sol for nft start!!");

    let system_program = &ctx.accounts.system_program;
    let bank_account = &mut ctx.accounts.bank_account;
    let pda_auth = &mut ctx.accounts.pda_auth;
    let sol_vault = &mut ctx.accounts.sol_vault;

    let client_amount = 0.01 * (client_ratio as f64);
    let bank_amount = 0.01 * (bank_ratio as f64);

    if bank_account.nft_amount.remained() > 0 {
        let seeds = &[
            b"sol-vault",
            pda_auth.to_account_info().key.as_ref(),
            &[bank_account.sol_vault_bump.unwrap()],
        ];
        let sol_vault_signer = &[&seeds[..]];

        // 1. transfer to client_account
        let cpi_accounts_to_client = system_program::Transfer {
            from: sol_vault.to_account_info(),
            to: ctx.accounts.client_account.to_account_info(),
        };
        let cpi_to_client = CpiContext::new_with_signer(
            system_program.to_account_info(),
            cpi_accounts_to_client,
            sol_vault_signer,
        );
        system_program::transfer(
            cpi_to_client,
            ((LAMPORTS_PER_SOL as f64) * client_amount) as u64,
        )?;

        // 2. transfer to bank_auth
        let cpi_accounts_to_bank_auth = system_program::Transfer {
            from: sol_vault.to_account_info(),
            to: ctx.accounts.bank_auth.to_account_info(),
        };
        let cpi_to_bank_auth = CpiContext::new_with_signer(
            system_program.to_account_info(),
            cpi_accounts_to_bank_auth,
            sol_vault_signer,
        );
        system_program::transfer(
            cpi_to_bank_auth,
            ((LAMPORTS_PER_SOL as f64) * bank_amount) as u64,
        )?;

        bank_account.nft_amount.decrease_one();

        msg!(
            "NFT remained amount: {}",
            bank_account.nft_amount.remained()
        );

        Ok(())
    } else {
        return err!(DepositForNftError::NoNftLeftError);
    }
}
