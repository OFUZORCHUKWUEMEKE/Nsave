use anchor_lang::prelude::*;
mod constants;
mod errors;
mod instructions;
mod state;
use instructions::*;
pub use state::*;

declare_id!("3nQpqWfTaTuUobguS1a5pUd5aguyUK7d6SDCnUWr8kmQ");

#[program]
pub mod nsave {
    use super::*;

    pub fn initialize_savings(
        ctx: Context<InitializeSavings>,
        name: String,
        description: String,
        is_sol: bool,
        savings_type: SavingsType,
        amount: u64,
        lock_duration: Option<i64>,
        unlock_price: Option<u64>,
    ) -> Result<()> {
        initialize(
            ctx,
            name,
            description,
            is_sol,
            savings_type,
            amount,
            lock_duration,
            unlock_price,
        );
        Ok(())
    }
}
