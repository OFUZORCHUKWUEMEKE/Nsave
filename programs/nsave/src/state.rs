use anchor_lang::prelude::*;

use crate::errors::NonceError;

#[account]
#[derive(InitSpace)]
pub struct ProtocolState {
    // pub payer: Pubkey,
    pub total_sol_saved: u64,
    pub total_usdc_saved: u64,
    pub last_updated: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct SavingsAccount {
    #[max_len(20)]
    pub name: String,
    pub amount: u64,
    #[max_len(50)]
    pub description: String,
    pub owner: Pubkey,
    pub bump: u8,
    pub is_active: bool,
    pub lock_duration: i64,
    pub created_at: i64,
    pub savings_type: SavingsType,
    pub is_sol: bool,
    pub unlock_price: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq, Clone, InitSpace)]
pub enum SavingsType {
    TimeLockedSavings,
    PriceLockedSavings,
}

impl SavingsAccount {
    pub fn is_locked(&self) -> Result<bool> {
        let current_time = Clock::get()?.unix_timestamp;
        let lock_end_time = self
            .created_at
            .checked_add(self.lock_duration)
            .ok_or(NonceError::NumericalOverflow)?;
        return Ok(current_time < lock_end_time);
    }

    pub fn get_remaining_time_formatted(&self) -> Result<String> {
        let mut remaining: i64;
        let current_time = Clock::get()?.unix_timestamp;

        let lock_end_time = self
            .created_at
            .checked_add(self.lock_duration)
            .ok_or(NonceError::NumericalOverflow)?;

        if current_time >= lock_end_time {
            remaining = 0
        }

        remaining = lock_end_time.checked_sub(current_time).unwrap();

        if remaining == 0 {
            return Ok("Unlocked".to_string());
        }

        let days = remaining / 86400;
        let hours = (remaining % 86400) / 3600;
        let minutes = (remaining % 3600) / 60;

        Ok(format!("{}d {}h {}m", days, hours, minutes))
    }
}
