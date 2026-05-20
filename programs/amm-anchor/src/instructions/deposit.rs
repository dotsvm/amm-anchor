use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{constants::*, errors::AmmError, state::Pool};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        seeds = [POOL_SEED, mint_x.key().as_ref(), mint_y.key().as_ref()],
        bump = pool.bump,
        has_one = vault_x,
        has_one = vault_y,
        has_one = lp_mint,
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut, token::mint = mint_x, token::authority = pool)]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(mut, token::mint = mint_y, token::authority = pool)]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [LP_MINT_SEED, pool.key().as_ref()],
        bump = pool.lp_bump,
    )]
    pub lp_mint: Account<'info, Mint>,

    #[account(mut, token::mint = mint_x, token::authority = user)]
    pub user_x: Account<'info, TokenAccount>,

    #[account(mut, token::mint = mint_y, token::authority = user)]
    pub user_y: Account<'info, TokenAccount>,

    #[account(mut, token::mint = lp_mint, token::authority = user)]
    pub user_lp: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
