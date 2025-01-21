use crate::{
    constants::*,
    errors::*,
    state::{ProtocolState, SavingsAccount, SavingsType},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Token},
    token_interface::{self, Mint, TokenAccount, TransferChecked},
};

#[derive(Accounts)]
#[instruction(name:String,description:String,savings_type:SavingsType,is_sol:bool)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds=[name.as_bytes(),signer.key().as_ref(),description.as_bytes()],
        bump=savings_account.bump
    )]
    pub savings_account: Box<Account<'info, SavingsAccount>>,
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
    pub protocol_state: Account<'info, ProtocolState>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority = signer
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_handler(
    ctx: Context<Deposit>,
    _name: String,
    _description: String,
    savings_type: SavingsType,
    is_sol: bool,
    amount: u64,
    time_lock: Option<i64>,
    unlock_price: Option<u64>,
) -> Result<()> {
    let vault_sol_account = &mut ctx.accounts.savings_account;
    let protocol_state = &mut ctx.accounts.protocol_state;
    let current_time = Clock::get()?.unix_timestamp;
    if is_sol == true {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: vault_sol_account.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_ctx, amount);
        protocol_state.total_sol_saved.checked_add(amount);
        protocol_state.last_updated = current_time;
    } else {
        let transfer_cpi_accounts = TransferChecked {
            from: ctx.accounts.user_ata.to_account_info(),
            to: ctx.accounts.token_vault_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, transfer_cpi_accounts);
        let decimals = ctx.accounts.mint.decimals;

        token_interface::transfer_checked(cpi_ctx, amount, decimals)?;
        protocol_state.total_usdc_saved.checked_add(amount);
        protocol_state.last_updated = current_time;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::nsave;

    use super::*;
    use anchor_lang::AccountSerialize;
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        instruction::Instruction,
        program_pack::Pack,
        signature::{Keypair, Signer},
        system_instruction, system_program,
        transaction::Transaction,
    };
    use std::str::FromStr;

    // Helper function to setup the program test environment
    async fn setup() -> (ProgramTestContext, Keypair) {
        let mut program_test = ProgramTest::new(
            "nsave", // Replace with your program name
            crate::id(),
            None, // Your program ID
        );

        // Add your program's accounts
        let user = Keypair::new();
        program_test.add_account(
            user.pubkey(),
            Account {
                lamports: 1_000_000_000, // 1 SOL
                ..Account::default()
            },
        );

        let mut context = program_test.start_with_context().await;
        (context, user)
    }

    async fn create_test_mint(
        context: &mut ProgramTestContext,
        authority: &Keypair,
        decimals: u8,
    ) -> (Pubkey, Pubkey) {
        let mint = Keypair::new();
        let mint_rent = context
            .banks_client
            .get_rent()
            .await
            .unwrap()
            .minimum_balance(spl_token::state::Mint::LEN);

        let tx = Transaction::new_signed_with_payer(
            &[
                system_instruction::create_account(
                    &context.payer.pubkey(),
                    &mint.pubkey(),
                    mint_rent,
                    spl_token::state::Mint::LEN as u64,
                    &spl_token::id(),
                ),
                spl_token::instruction::initialize_mint(
                    &spl_token::id(),
                    &mint.pubkey(),
                    &authority.pubkey(),
                    None,
                    decimals,
                )
                .unwrap(),
            ],
            Some(&context.payer.pubkey()),
            &[&context.payer, &mint],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        let associated_token_account = spl_associated_token_account::get_associated_token_address(
            &authority.pubkey(),
            &mint.pubkey(),
        );

        let tx = Transaction::new_signed_with_payer(
            &[
                spl_associated_token_account::instruction::create_associated_token_account(
                    &context.payer.pubkey(),
                    &authority.pubkey(),
                    &mint.pubkey(),
                    &spl_token::id(),
                ),
            ],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        (mint.pubkey(), associated_token_account)
    }

    fn your_instruction_data() -> Vec<u8> {
        #[derive(AnchorSerialize, AnchorDeserialize)]
        pub enum SavingsInstruction {
            Deposit {
                name: String,
                description: String,
                savings_type: SavingsType,
                is_sol: bool,
                amount: u64,
                time_lock: Option<i64>,
                unlock_price: Option<u64>,
            },
            // ... other instructions
        }
        let ix = SavingsInstruction::Deposit {
            name: "Test Account".to_string(),
            description: "Test Description".to_string(),
            savings_type: SavingsType::TimeLockedSavings,
            is_sol: true,        // true for SOL deposits
            amount: 100_000_000, // Amount in lamports (0.1 SOL)
            time_lock: Some(60),
            unlock_price: None,
        };

        ix.try_to_vec().unwrap()
    }

    #[tokio::test]
    async fn test_deposit() {
        let (mut context, user) = setup().await;
        let name = "Test Account".to_string();
        let description = "Test Description".to_string();

        let (savings_account_pda, bump) = Pubkey::find_program_address(
            &[
                name.as_bytes(),
                user.pubkey().as_ref(),
                description.as_bytes(),
            ],
            &crate::id(),
        );

        // Calculate PDA for token vault
        let (token_vault_pda, vault_bump) =
            Pubkey::find_program_address(&[b"vault", savings_account_pda.as_ref()], &crate::id());

        let (protocol_state_pda, protocol_bump) =
            Pubkey::find_program_address(&[b"protocol", user.pubkey().as_ref()], &crate::id());

        let deposit_amount = 100_000_000; // 0.1 SOL

        let ix = Instruction {
            program_id: crate::id(),
            accounts: vec![
                AccountMeta::new(user.pubkey(), true),
                AccountMeta::new(savings_account_pda, false),
                AccountMeta::new(token_vault_pda, false),
                AccountMeta::new(protocol_state_pda, false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data: your_instruction_data(), // Implement this based on your instruction data
        };
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user.pubkey()),
            &[&user],
            context.last_blockhash,
        );

        // Process the transaction
        context.banks_client.process_transaction(tx).await.unwrap();

        // Verify the deposit
        let savings_account = context
            .banks_client
            .get_account(savings_account_pda)
            .await
            .unwrap()
            .unwrap();

        let savings_data =
            SavingsAccount::try_deserialize(&mut savings_account.data.as_ref()).unwrap();
        assert_eq!(savings_data.amount, deposit_amount);
        assert_eq!(savings_data.is_sol, true);
        assert_eq!(savings_data.owner, user.pubkey());
    }
}
