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
// use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
// use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

#[derive(Accounts)]
#[instruction(name:String,description:String,savings_type:SavingsType,is_sol:bool)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        close=signer,
        seeds=[name.as_bytes(),signer.key().as_ref(),description.as_bytes()],
        bump=savings_account.bump
    )]
    pub savings_account: Box<Account<'info, SavingsAccount>>,
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds=[b"vault",signer.key().as_ref()],
        bump
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
        associated_token::authority = savings_account,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    // #[account(address = Pubkey::from_str(SOL_USD_FEED_ID).unwrap() @ NonceError::InvalidPriceFeed)]
    // pub price_feed: AccountInfo<'info>,
    // pub price_update: Account<'info, PriceUpdateV2>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_handler(
    ctx: Context<Withdraw>,
    amount: u64,
    unlock_price: Option<u64>,
    lock_duration: Option<i64>,
) -> Result<()> {
    let savings_account = &ctx.accounts.savings_account;
    // let price_update = &mut ctx.accounts.price_update;

    let seeds = &[
        ctx.accounts.savings_account.name.as_bytes(),
        ctx.accounts.signer.to_account_info().key.as_ref(),
        ctx.accounts.savings_account.description.as_bytes(),
        &[ctx.accounts.savings_account.bump],
    ];
    let signer_seeds = [&seeds[..]];
    let signer_account = &mut ctx.accounts.signer;

    if savings_account.is_sol == true {
        let current_timestamp = Clock::get()?.unix_timestamp;
        if current_timestamp >= savings_account.created_at + lock_duration.unwrap() {
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: savings_account.to_account_info(),
                    to: signer_account.to_account_info(),
                },
                &signer_seeds,
            );
            anchor_lang::system_program::transfer(cpi_ctx, amount);
        } else {
            return Err(NonceError::FundsStillLocked.into());
        }
    } else {
        let current_timestamp = Clock::get()?.unix_timestamp;
        if current_timestamp >= savings_account.created_at + lock_duration.unwrap() {
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let decimals = ctx.accounts.mint.decimals;
            let transfer_accounts = TransferChecked {
                from: ctx.accounts.token_vault_account.to_account_info(),
                to: ctx.accounts.user_ata.to_account_info(),
                authority: savings_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            };
            let ctx = CpiContext::new_with_signer(cpi_program, transfer_accounts, &signer_seeds);
            token_interface::transfer_checked(ctx, amount, decimals)?;
        } else {
            return Err(NonceError::FundsStillLocked.into());
        }
    }

    Ok(())
}
