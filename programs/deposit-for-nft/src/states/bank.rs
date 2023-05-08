use anchor_lang::prelude::*;

#[account]
pub struct BankAccount {
    pub bank_auth: Pubkey,
    pub auth_bump: u8,
    pub sol_vault_bump: Option<u8>,
    pub nft_amount: u16,
}

impl BankAccount {
    pub const LEN: usize = 8 + 32 + 1 + 1 + 1 + 2;
}