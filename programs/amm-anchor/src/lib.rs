use anchor_lang::prelude::*;

declare_id!("3DLfmLuhAPRVbVFKTake4t2Z47M2bUGsM9oPaH9oDc8w");

#[program]
pub mod amm_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
