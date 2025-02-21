use std::str::FromStr;

use crate::{
    constants::*,
    errors::NonceError,
    state::{SavingsAccount, SavingsType},
    ProtocolState,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TransferChecked},
};

#[derive(Accounts)]
#[instruction(name:String,description:String,savings_type:SavingsType,is_sol:bool)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        // close=signer,
        seeds=[name.as_bytes(),signer.key().as_ref(),description.as_bytes()],
        bump=savings_account.bump
    )]
    pub savings_account: Box<Account<'info, SavingsAccount>>,
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
       associated_token::mint = mint,
       associated_token::authority = savings_account
    )]
    pub token_vault_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,
    #[account(
        mut,
        seeds=[b"protocol",signer.key().as_ref()],
        bump
    )]
    pub protocol_state: Box<Account<'info, ProtocolState>>,
    #[account(
        mut,associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

// CHATGPT

pub fn withdraw_handler(
    ctx: Context<Withdraw>,
    amount: u64,
    lock_duration: i64, // Lock duration in seconds
) -> Result<()> {
    let savings_account = &ctx.accounts.savings_account;
    let current_timestamp = Clock::get()?.unix_timestamp;

    // Check if the lock duration has elapsed
    if current_timestamp < savings_account.created_at + lock_duration {
        return Err(NonceError::FundsStillLocked.into());
    }

    // Ensure the savings account has enough balance
    if savings_account.amount < amount {
        return Err(NonceError::InsufficientBalance.into());
    }

    let signer_seeds = &[
        savings_account.name.as_bytes(),
        ctx.accounts.signer.to_account_info().key.as_ref(),
        savings_account.description.as_bytes(),
        &[savings_account.bump],
    ];

    let signer_seeds_ref: &[&[&[u8]]] = &[signer_seeds];

    if savings_account.is_sol {
        // Transfer SOL from savings_account to signer
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.savings_account.to_account_info(),
                to: ctx.accounts.signer.to_account_info(),
            },
            signer_seeds_ref,
        );

        anchor_lang::system_program::transfer(cpi_ctx, amount)?;
    } else {
        // Ensure the token_vault_account has enough balance
        let vault_balance = ctx.accounts.token_vault_account.amount;
        if vault_balance < amount {
            return Err(NonceError::InsufficientBalance.into());
        }
        // Transfer tokens from token_vault_account to user_ata
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let decimals = ctx.accounts.mint.decimals;

        let transfer_accounts = TransferChecked {
            from: ctx.accounts.token_vault_account.to_account_info(),
            to: ctx.accounts.user_ata.to_account_info(),
            authority: ctx.accounts.savings_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, transfer_accounts, signer_seeds_ref);
        token_interface::transfer_checked(cpi_ctx, amount, decimals)?;
    }

    // Update protocol state
    let protocol_state = &mut ctx.accounts.protocol_state;
    if savings_account.is_sol {
        protocol_state.total_sol_saved = protocol_state.total_sol_saved.saturating_sub(amount);
    } else {
        protocol_state.total_usdc_saved = protocol_state.total_usdc_saved.saturating_sub(amount);
    }
    protocol_state.last_updated = current_timestamp;
    // Update savings account balance
    ctx.accounts.savings_account.amount = savings_account.amount.saturating_sub(amount);

    Ok(())
}
