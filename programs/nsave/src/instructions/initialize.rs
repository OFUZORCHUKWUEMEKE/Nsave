use crate::constants::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface;
use anchor_spl::token_interface::Mint;

#[derive(Accounts)]
#[instruction(name:String,description:String,savings_type:SavingsType)]
pub struct InitializeSavings<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        seeds=[b"protocol"],
        bump,
        payer=signer,
        space=DISCRIMINATOR + ProtocolState::INIT_SPACE
    )]
    pub protocol: Account<'info, ProtocolState>,
    #[account(
        init_if_needed,
        payer=signer,
        token::authority= savings_account,
        token::mint = mint,
        seeds=[b"vault",savings_account.key().as_ref()],
        bump
    )]
    pub token_vault_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        init,
        seeds=[name.as_bytes(),signer.key().as_ref(),description.as_bytes()],
        bump,
        payer=signer,
        space=DISCRIMINATOR + SavingsAccount::INIT_SPACE,
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}
