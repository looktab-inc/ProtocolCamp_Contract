use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct BankAccount {
    pub bank_auth: Pubkey,
    pub auth_bump: u8,
    pub sol_vault_bump: Option<u8>,
    // pub nft_amount: u16,
    pub nft_amount: NftAmount,
}

impl BankAccount {
    pub const LEN: usize = 8 + 32 + 1 + 1 + 1 + 2 + 2;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct NftAmount {
    total: u16,
    remained: u16,
}

impl NftAmount {
    pub fn new(total_amount: u16, remained_amount: u16) -> Self {
        Self {
            total: total_amount,
            remained: remained_amount,
        }
    }

    pub fn increase_one(&mut self) {
        self.total += 1;
        self.remained += 1;
    }

    pub fn decrease_one(&mut self) {
        if self.remained > 0 {
            self.remained -= 1;
        }
    }

    pub fn total(&self) -> u16 {
        self.total
    }

    pub fn remained(&self) -> u16 {
        self.remained
    }
}
