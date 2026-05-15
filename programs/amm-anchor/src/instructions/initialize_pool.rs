use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount}
};

use crate::{constants::*, errors::AmmError, state::Pool};

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        space = 8 + Pool::INIT_SPACE,
        seeds = [POOL_SEED, mint_x.key().as_ref(), mint_y.key().as_ref()],
        bump,
        constraint = mint_x.key() < mint_y.key() @AmmError::MintsNotSorted,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_x,
        associated_token::authority = pool,
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_y,
        associated_token::authority = pool,
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        seeds = [LP_MINT_SEED, pool.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = pool,        
    )]
    pub lp_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializePool>, fee_bps: u16) -> Result<()> {
    require!(fee_bps <= MAX_FEE_BPS, AmmError::FeeToHigh);
    require_keys_neq!(
        ctx.accounts.mint_x.key(),
        ctx.accounts.mint_y.key(),
        AmmError::SameMint
    );

    let pool = &mut ctx.accounts.pool;
    pool.mint_x = ctx.accounts.mint_x.key();
    pool.mint_y = ctx.accounts.mint_y.key();
    pool.vault_x = ctx.accounts.vault_x.key();
    pool.vault_y = ctx.accounts.vault_y.key();
    pool.lp_mint = ctx.accounts.lp_mint.key();
    pool.fee_bps = fee_bps;
    pool.bump = ctx.bumps.pool;
    pool.lp_bump = ctx.bumps.lp_mint;

    Ok(())
}