use anchor_lang::{prelude::*, system_program};

use crate::{constants::DEPOSIT_PER_NFT, states::BankAccount, DepositForNftError};

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
    #[account(mut)]
    pub recommender_account: SystemAccount<'info>,
}

pub fn handle(ctx: Context<WithdrawSolForNft>, recommender_ratio: u8, client_ratio: u8) -> Result<()> {
    msg!("withdraw sol for nft start!!");

    let system_program = &ctx.accounts.system_program;
    let bank_account = &mut ctx.accounts.bank_account;
    let pda_auth = &mut ctx.accounts.pda_auth;
    let sol_vault = &mut ctx.accounts.sol_vault;

    let recommender_ratio = (DEPOSIT_PER_NFT as f64 * (0.01 * recommender_ratio as f64)) as u64;
    let client_amount = (DEPOSIT_PER_NFT as f64 * (0.01 * client_ratio as f64)) as u64;

    msg!("recommender_ratio : {}", recommender_ratio);
    msg!("Client_amount : {}", client_amount);

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
        system_program::transfer(cpi_to_client, client_amount)?;

        // 2. transfer to bank_auth
        let cpi_accounts_to_bank_auth = system_program::Transfer {
            from: sol_vault.to_account_info(),
            to: ctx.accounts.recommender_account.to_account_info(),
        };
        let cpi_to_bank_auth = CpiContext::new_with_signer(
            system_program.to_account_info(),
            cpi_accounts_to_bank_auth,
            sol_vault_signer,
        );
        system_program::transfer(cpi_to_bank_auth, recommender_ratio)?;

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
