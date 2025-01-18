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
    lock_duration: Option<i64>,
    unlock_price: Option<u64>,
    savings_type: SavingsType,
    amount: u64,
) -> Instruction {
    let (protocol, _) = Pubkey::find_program_address(&[b"protocol", signer.as_ref()], &program_id);

    let (savings_pubkey, _) =
        Pubkey::find_program_address(&[b"name", signer.as_ref(), b"description"], &program_id);

    let (vault_pubkey, _) =
        Pubkey::find_program_address(&[b"vault", savings_pubkey.as_ref()], &program_id);

    Instruction {
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
            unlock_price,
        }),
    };
}
