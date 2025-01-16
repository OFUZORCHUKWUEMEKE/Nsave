use std::str::FromStr;

use crate::{
    constants::*,
    errors::NonceError,
    state::{SavingsAccount, SavingsType},
    ProtocolState,
};
use anchor_lang::{prelude::*, solana_program::address_lookup_table::instruction};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TransferChecked},
};
use pyth_sdk_solana;
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
    pub savings_account: Account<'info, SavingsAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds=[b"vault",signer.key().as_ref()],
        bump
    )]
    pub token_vault_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        seeds=[b"protocol",signer.key().as_ref()],
        bump
    )]
    pub protocol_state: Account<'info, ProtocolState>,
    #[account(
        mut,associated_token::mint = mint,
        associated_token::authority = savings_account,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(address = Pubkey::from_str(SOL_USD_FEED_ID).unwrap() @ NonceError::InvalidPriceFeed)]
    pub price_feed: AccountInfo<'info>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
