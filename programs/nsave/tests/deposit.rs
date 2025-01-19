pub mod helpers;

use {
    anchor_lang::{AccountDeserialize, AnchorSerialize, Space},
    helpers::{spl_token_helpers::*, *},
    nsave::{SavingsAccount, SavingsType},
    rand::Rng,
    solana_program_test::*,
    solana_sdk::{
        account::{Account as SolanaAccount, AccountSharedData},
        clock::Clock,
        native_token::LAMPORTS_PER_SOL,
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        transaction::Transaction,
    },
    spl_token::state::{Account as TokenAccount, AccountState, Mint},
    std::u64,
};

#[tokio::test]
async fn test_successful_deposit() {
    let mut test = ProgramTest::new("nsave", nsave::id(), None);

    test.set_compute_max_units(100_000);

    let maker = Keypair::new();

    let seed: u64 = rand::thread_rng().gen();

    let name = "Emeke";
    let description = "Savings";
    let is_sol = true;
    let lock_duration = 60;
    let unlock_price: u64 = 10;
    let savings_type = SavingsType::TimeLockedSavings;
    let amount = 1000;
    let mint = Keypair::new().pubkey();
    let mut mint_data = vec![0u8; Mint::LEN];
    Mint {
        is_initialized: true,
        decimals: 6,
        mint_authority: COption::None,
        supply: 100_000,
        ..Mint::default()
    }
    .pack_into_slice(&mut mint_data);
    test.add_account(
        mint,
        SolanaAccount {
            lamports: u32::MAX as u64,
            data: mint_data,
            owner: spl_token::id(),
            ..SolanaAccount::default()
        },
    );
    let (protocol, _) =
        Pubkey::find_program_address(&[b"protocol", maker.pubkey().as_ref()], &nsave::id());

    let (savings_pubkey, bump) = Pubkey::find_program_address(
        &[
            name.as_bytes(),
            maker.pubkey().as_ref(),
            description.as_bytes(),
        ],
        &nsave::id(),
    );
    let (token_vault, _) =
        Pubkey::find_program_address(&[b"vault", savings_pubkey.as_ref()], &nsave::id());

    // let mut account_data = vec![0u8; TokenAccount::LEN];
    let mut account_data = vec![0u8; TokenAccount::LEN];
    TokenAccount {
        mint,
        // check this owner of the account
        owner: token_vault,
        amount,
        state: AccountState::Initialized,
        ..TokenAccount::default()
    }
    .pack_into_slice(&mut account_data);

    test.add_account(
        token_vault,
        SolanaAccount {
            lamports: u32::MAX as u64,
            data: account_data,
            owner: spl_token::id(),
            ..SolanaAccount::default()
        },
    );

    let mut context = test.start_with_context().await;
    let _ = airdrop(
        &mut context.banks_client,
        &context.payer,
        &maker.pubkey(),
        3 * LAMPORTS_PER_SOL,
    )
    .await;
    // Get the current timestamp
    let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();
    let current_time = clock.unix_timestamp;
    let (savings_pubkey, bump) = Pubkey::find_program_address(
        &[
            name.as_bytes(),
            maker.pubkey().as_ref(),
            description.as_bytes(),
        ],
        &nsave::id(),
    );
    let savings = SavingsAccount {
        name: name.to_string(),
        amount,
        description: description.to_string(),
        owner: maker.pubkey(),
        bump,
        is_active: true,
        lock_duration: 60,
        unlock_price: 10,
        created_at: current_time,
        is_sol: true,
        savings_type: SavingsType::TimeLockedSavings,
    };
    let data = savings.try_to_vec().unwrap();
    let mut account =
        AccountSharedData::new(u32::MAX as u64, SavingsAccount::INIT_SPACE, &nsave::id());
    account.set_data_from_slice(&data);
    context.set_account(&savings_pubkey, &account);
    let mut transaction = Transaction::new_with_payer(
        &[deposit(
            nsave::id(),
            mint,
            spl_token::id(),
            maker.pubkey(),
            name.to_string(),
            description.to_string(),
            savings_type,
            is_sol,
            savings_pubkey,
            token_vault,
            protocol,
            amount,
            Some(lock_duration),
            Some(unlock_price),
        )],
        Some(&context.payer.pubkey()),
    );

    transaction.sign(&[&context.payer, &maker], context.last_blockhash);
    let result = context.banks_client.process_transaction(transaction).await;

    let savings_account = context
        .banks_client
        .get_account(savings_pubkey)
        .await
        .unwrap()
        .unwrap(); // assert!(savings_account.is_some(), "Escrow account should still exist");

    // let savings_account = MySavingsAccount::try_deserialize(&mut &account_data[..])?;
    let savings_data = SavingsAccount::try_deserialize(&mut &savings_account.data[..]).unwrap();

    assert_eq!(savings_data.is_active, false);
}
