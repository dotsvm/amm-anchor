use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    #[msg("Mints must be passed in canonical (sorted) order")]
    MintsNotSorted,

    #[msg("Mints must be different")]
    SameMint,

    #[msg("Fee exceeds maximum")]
    FeeTooHigh,

    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Slippage exceeded")]
    SlippageExceeded,

    #[msg("Insufficient Liquidity")]
    InsufficientLiquidity,
}
