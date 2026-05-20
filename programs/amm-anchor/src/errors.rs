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
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
}

pub fn map_curve_err(e: cp_curve::CurveError) -> Error {
    use cp_curve::CurveError as C;
    match e {
        C::Overflow => AmmError::MathOverflow.into(),
        C::ZeroInput => AmmError::ZeroAmount.into(),
        C::EmptyPool => AmmError::InsufficientLiquidity.into(),
        C::InvalidFee => AmmError::FeeTooHigh.into(),
        C::InsufficientLpSupply => AmmError::InsufficientLiquidity.into(),
    }
}
