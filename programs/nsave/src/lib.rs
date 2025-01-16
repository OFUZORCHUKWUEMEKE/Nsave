use anchor_lang::prelude::*;
mod constants;
mod errors;
mod instructions;
mod state;
use instructions::*;
use state::*;

declare_id!("3nQpqWfTaTuUobguS1a5pUd5aguyUK7d6SDCnUWr8kmQ");

#[program]
pub mod nsave {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
