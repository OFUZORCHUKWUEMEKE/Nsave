pub mod helpers;

use {
    anchor_lang::AccountDeserialize,
    helpers::{spl_token_helpers::*, *},
    nsave::{SavingsAccount, SavingsType},
    rand::Rng,
    solana_program_test::*,
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
        transaction::Transaction,
    },
    std::u64,
};

#[tokio::test]
async fn test_successful_initiate() {
    let mut test = ProgramTest::new("nsave", nsave::id(), None);

    test.set_compute_max_units(100_000);

    let maker = Keypair::new();

    let seed: u64 = rand::thread_rng().gen();

    let (mut banks_client, payer, recent_blockhash) = test.start().await;

    let _ = airdrop(
        &mut banks_client,
        &payer,
        &maker.pubkey(),
        4 * LAMPORTS_PER_SOL,
    )
    .await;

    let mint = create_mint(&mut banks_client, &payer, None).await.unwrap();

    let name = "Emeke";
    let description = "Savings";
    let is_sol = true;
    let lock_duration = 60;
    let unlock_price: u64 = 10;
    let savings_type = SavingsType::TimeLockedSavings;
    let amount = 1000;

    let _ =
        create_and_mint_to_token_account(&mut banks_client, mint, &payer, maker.pubkey(), 100_000)
            .await;

    let mut transaction = Transaction::new_with_payer(
        &[initialize(
            nsave::id(),
            mint,
            maker.pubkey(),
            spl_token::id(),
            name.to_string(),
            description.to_string(),
            is_sol,
            Some(lock_duration),
            Some(unlock_price),
            SavingsType::TimeLockedSavings,
            amount,
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &maker], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let (savings_pubkey, _) = Pubkey::find_program_address(
        &[
            name.as_bytes(),
            maker.pubkey().as_ref(),
            description.as_bytes(),
        ],
        &nsave::id(),
    );

    let savings = banks_client
        .get_account(savings_pubkey)
        .await
        .unwrap()
        .unwrap();

    // Deserialize the account data
    let mut account_data = savings.data.as_ref();
    let escrow_account = SavingsAccount::try_deserialize(&mut account_data).unwrap();

    assert_eq!(escrow_account.owner, maker.pubkey());
    assert_eq!(escrow_account.amount, 1000);
}
