pub mod spl_token_helpers;

use {
    anchor_lang::error::ERROR_CODE_OFFSET,
    nsave::SavingsType,
    solana_program_test::{BanksClient, BanksClientError, ProgramTestContext},
    solana_sdk::{
        clock::Clock,
        instruction::{Instruction, InstructionError},
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        system_instruction::transfer,
        system_program,
        transaction::{Transaction, TransactionError},
    },
    spl_associated_token_account::get_associated_token_address_with_program_id,
};

#[allow(dead_code)]
pub async fn airdrop(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    receiver: &Pubkey,
    amount: u64,
) -> Result<(), BanksClientError> {
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&payer.pubkey(), receiver, amount)],
        Some(&payer.pubkey()),
        &[payer],
        banks_client.get_latest_blockhash().await?,
    );

    banks_client.process_transaction(transaction).await
}

pub fn initialize(
    program_id: Pubkey,
    mint: Pubkey,
    signer: Pubkey,
    token_program_id: Pubkey,
    name: String,
    description: String,
    is_sol: bool,
    lock_duration: i64,
    unlock_price: Option<u64>,
    savings_type: SavingsType,
    amount: u64,
) -> Instruction {
    let (protocol, _) = Pubkey::find_program_address(&[b"protocol", signer.as_ref()], &program_id);

    let (savings_pubkey, _) = Pubkey::find_program_address(
        &[name.as_bytes(), signer.as_ref(), description.as_bytes()],
        &program_id,
    );

    let (vault_pubkey, _) =
        Pubkey::find_program_address(&[b"vault", savings_pubkey.as_ref()], &program_id);

    return Instruction {
        program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &nsave::accounts::InitializeSavings {
                signer,
                mint,
                protocol,
                token_vault_account: vault_pubkey,
                savings_account: savings_pubkey,
                token_program: token_program_id,
                system_program: system_program::id(),
                associated_token_program:spl_associated_token_account::id()
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&nsave::instruction::InitializeSavings {
            name,
            description,
            is_sol,
            savings_type,
            amount,
            lock_duration,
        }),
    };
}

pub fn deposit(
    program_id: Pubkey,
    mint: Pubkey,
    token_program: Pubkey,
    signer: Pubkey,
    name: String,
    description: String,
    savings_type: SavingsType,
    is_sol: bool,
    savings_account: Pubkey,
    token_vault_account: Pubkey,
    protocol_state: Pubkey,
    amount: u64,
    _time_lock: Option<i64>,
    _unlock_price: Option<u64>,
) -> Instruction {
    let user_ata = get_associated_token_address_with_program_id(&signer, &mint, &token_program);
    return Instruction {
        program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &nsave::accounts::Deposit {
                signer,
                savings_account,
                token_vault_account,
                protocol_state,
                mint,
                user_ata,
                token_program,
                associated_token_program: spl_associated_token_account::id(),
                system_program: system_program::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&nsave::instruction::DepositSavings {
            _name: name,
            _description: description,
            _savings_type: savings_type,
            is_sol,
            amount,
            _time_lock: _time_lock,
            _unlock_price: _unlock_price,
        }),
    };
}
