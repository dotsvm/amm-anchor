use anchor_lang::prelude::*;

#[constant]
pub const POOL_SEED: &[u8] = b"pool";

#[constant]
pub const LP_MINT_SEED: &[u8] = b"lp";

pub const MAX_FEE_BPS: u16 = 10_000;