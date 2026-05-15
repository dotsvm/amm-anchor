use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    #[msg("Mint must be passed in canonical (sorted) order")]
    MintsNotSorted,

    #[msg("Mint must be different")]
    SameMint,

    #[msg("Fee exceeds maximum")]
    FeeToHigh,

    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Slippage exceeded")]
    SlippageExceeded,

    #[msg("Insufficient Liquidity")]
    InsufficientLiquidity,
}
