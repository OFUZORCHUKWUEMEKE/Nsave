use std::str::FromStr;

use anchor_lang::prelude::*;
mod constants;
mod errors;
pub mod instructions;
mod state;
mod utils;
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
        lock_duration: i64,
    ) -> Result<()> {
        initialize(
            ctx,
            name,
            description,
            is_sol,
            savings_type,
            amount,
            lock_duration,
        )?;
        Ok(())
    }

    pub fn deposit_savings(
        ctx: Context<Deposit>,
        _name: String,
        _description: String,
        _savings_type: SavingsType,
        is_sol: bool,
        amount: u64,
        _time_lock: Option<i64>,
        _unlock_price: Option<u64>,
    ) -> Result<()> {
        deposit_handler(
            ctx,
            _name,
            _description,
            _savings_type,
            is_sol,
            amount,
            _time_lock,
            _unlock_price,
        )?;
        Ok(())
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        _name: String,
        _description: String,
        _savings_type: SavingsType,
        _is_sol: bool,
        amount: u64,
        unlock_price: Option<u64>,
        lock_duration: i64,
    ) -> Result<()> {
        withdraw_handler(ctx, amount, lock_duration)?;
        Ok(())
    }
}
