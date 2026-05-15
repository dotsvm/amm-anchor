use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;
pub mod instructions;

pub use constants::*;
pub use errors::*;
pub use state::*;
pub use instructions::*;

declare_id!("3DLfmLuhAPRVbVFKTake4t2Z47M2bUGsM9oPaH9oDc8w");

#[program]
pub mod amm_anchor {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, fee_bps: u16) -> Result<()> {
        instructions::initialize_pool::handler(ctx, fee_bps)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
