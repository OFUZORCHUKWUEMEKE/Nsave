use anchor_lang::prelude::Pubkey as AnchorPubkey;
use solana_program::pubkey::Pubkey as ProgramPubkey;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use nsave::instructions::initialize;
use nsave::{SavingsAccount, SavingsType, ProtocolState};

#[tokio::test]
async fn test_initialize_savings() {
    // Set up the test environment
    let program_id = ProgramPubkey::new_unique();
    let mut program_test = ProgramTest::new("nsave", program_id,None);

    // Create a mock signer (user)
    let signer = Keypair::new();
    let signer_pubkey: AnchorPubkey = AnchorPubkey::new_from_array(signer.pubkey().to_bytes()); // Convert to AnchorPubkey

    // Create a mock mint account (for token savings)
    let mint = Keypair::new();
    let mint_pubkey: AnchorPubkey = AnchorPubkey::new_from_array(mint.pubkey().to_bytes()); // Convert to AnchorPubkey

    // Create a mock protocol state account
    let protocol_state = Keypair::new();
    let protocol_state_pubkey: AnchorPubkey = AnchorPubkey::new_from_array(protocol_state.pubkey().to_bytes()); // Convert to AnchorPubkey

    // Create a mock savings account
    let savings_account = Keypair::new();
    let savings_account_pubkey: AnchorPubkey = AnchorPubkey::new_from_array(savings_account.pubkey().to_bytes()); // Convert to AnchorPubkey

    // Add the mock accounts to the test environment
    program_test.add_account(signer.pubkey(), solana_sdk::account::Account::new(0, 0, &program_id));
    program_test.add_account(mint.pubkey(), solana_sdk::account::Account::new(0, 0, &program_id));
    program_test.add_account(protocol_state.pubkey(), solana_sdk::account::Account::new(0, 0, &program_id));
    program_test.add_account(savings_account.pubkey(), solana_sdk::account::Account::new(0, 0, &program_id));

    // Build the transaction
    // let transaction = Transaction::new_with_payer(
    //     &[initialize(
    //         savings_account_pubkey,
    //         signer_pubkey,
    //         mint_pubkey,
    //         protocol_state_pubkey,
    //         "Test Savings".to_string(),
    //         "Test Description".to_string(),
    //         false, // is_sol
    //         SavingsType::TimeLockedSavings,
    //         1000, // amount
    //         3600, // lock_duration (1 hour)
    //     )],
    //     Some(&signer.pubkey()),
    // );
    // let transaction = Transaction::new_with_payer(&[initialize(

    // )],  Some(&signer.pubkey()))

    // Sign and process the transaction
    // transaction.sign(&[&signer], program_test.last_blockhash);
    // program_test.process_transaction(transaction).await.unwrap();

    // // Verify the savings account state
    // let updated_account = program_test.get_account(savings_account.pubkey()).await.unwrap();
    // let updated_savings_account: SavingsAccount = SavingsAccount::try_from_slice(&updated_account.data).unwrap();

    // assert_eq!(updated_savings_account.name, "Test Savings");
    // assert_eq!(updated_savings_account.description, "Test Description");
    // assert_eq!(updated_savings_account.savings_type, SavingsType::TimeLockedSavings);
    // assert_eq!(updated_savings_account.is_sol, false);
    // assert_eq!(updated_savings_account.owner, signer_pubkey);
    // assert_eq!(updated_savings_account.amount, 1000);
    // assert_eq!(updated_savings_account.lock_duration, 3600);
    // assert!(updated_savings_account.created_at > 0);
}