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

    let (savings_pubkey, _) = Pubkey::find_program_address(
        &[
            name.as_bytes(),
            maker.pubkey().as_ref(),
            description.as_bytes(),
        ],
        &nsave::id(),
    );
}
