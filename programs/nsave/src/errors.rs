use anchor_lang::prelude::*;

#[error_code]
pub enum NonceError {
    #[msg("Savings account is inactive")]
    SavingsInactive,
    #[msg("Funds are still Locked")]
    FundsStillLocked,
    #[msg("Unauthorized access to savings Account")]
    Unauthorized,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    #[msg("Mathematical Overflow")]
    Overflow,
    #[msg("USD Price not yet reached")]
    PriceNotReached,
    #[msg("Invalid Price Feed")]
    InvalidPriceFeed,
    #[msg("Numerical Overflow")]
    NumericalOverflow,
    #[msg("Insufficient balance")]
    InsufficientBalance
}