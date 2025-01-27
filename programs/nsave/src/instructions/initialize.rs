use crate::constants::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface;
use anchor_spl::token_interface::Mint;

#[derive(Accounts)]
#[instruction(name:String,description:String,is_sol:bool)]
pub struct InitializeSavings<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init_if_needed,
        seeds=[b"protocol",signer.key().as_ref()],
        bump,
        payer=signer,
        space=DISCRIMINATOR + ProtocolState::INIT_SPACE
    )]
    pub protocol: Box<Account<'info, ProtocolState>>,
    #[account(
        init_if_needed,
        payer=signer,
        associated_token::authority= savings_account,
        associated_token::mint = mint,
    )]
    pub token_vault_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,
    #[account(
        init,
        seeds=[name.as_bytes(),signer.key().as_ref(),description.as_bytes()],
        bump,
        payer=signer,
        space=DISCRIMINATOR + SavingsAccount::INIT_SPACE,
    )]
    pub savings_account: Box<Account<'info, SavingsAccount>>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn initialize(
    ctx: Context<InitializeSavings>,
    name: String,
    description: String,
    is_sol: bool,
    savings_type: SavingsType,
    amount: u64,
    lock_duration: i64,
) -> Result<()> {
    let savings_account = &mut ctx.accounts.savings_account;
    // let protocol_account = &mut ctx.accounts.protocol;
    savings_account.name = name;
    savings_account.description = description;
    savings_account.savings_type = savings_type;
    savings_account.is_sol = is_sol;
    savings_account.owner = ctx.accounts.signer.key();
    savings_account.bump = ctx.bumps.savings_account;
    savings_account.created_at = Clock::get()?.unix_timestamp;
    if savings_account.amount > 0 {
        let new = savings_account.amount.checked_add(amount).unwrap();
        savings_account.amount = new;
    } else {
        savings_account.amount = amount;
    }
    savings_account.lock_duration = lock_duration;
    Ok(())
}
