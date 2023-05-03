use anchor_lang::error_code;

#[error_code]
pub enum DepositForNftError {
    #[msg("Failed to initialize contract.")]
    InitError,
}