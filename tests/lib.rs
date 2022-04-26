use borsh::BorshDeserialize;
// use helloworld::{process_instruction, GreetingAccount};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token::state::{Mint};//Account, 
use solana_program::{hash::Hash, program_pack::Pack, system_instruction};
use predictchain::{EventPDA, process_instruction};

use std::mem;



// #[tokio::test]
// async fn test_helloworld() {
//     let program_id = Pubkey::new_unique();
//     let greeted_pubkey = Pubkey::new_unique();

//     let mut program_test = ProgramTest::new(
//         "predictchain", // Run the BPF version with `cargo test-bpf`
//         program_id,
//         processor!(process_instruction), // Run the native version with `cargo test`
//     );
//     program_test.add_account(
//         greeted_pubkey,
//         Account {
//             lamports: 5,
//             data: vec![0_u8; mem::size_of::<u32>()],
//             owner: program_id,
//             ..Account::default()
//         },
//     );
//     let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

//     // Verify account has zero greetings
//     let greeted_account = banks_client
//         .get_account(greeted_pubkey)
//         .await
//         .expect("get_account")
//         .expect("greeted_account not found");
//     assert_eq!(
//         GreetingAccount::try_from_slice(&greeted_account.data)
//             .unwrap()
//             .counter,
//         0
//     );

//     // Greet once
//     let mut transaction = Transaction::new_with_payer(
//         &[Instruction::new_with_bincode(
//             program_id,
//             &[0], // ignored but makes the instruction unique in the slot
//             vec![AccountMeta::new(greeted_pubkey, false)],
//         )],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[&payer], recent_blockhash);
//     banks_client.process_transaction(transaction).await.unwrap();

//     // Verify account has one greeting
//     let greeted_account = banks_client
//         .get_account(greeted_pubkey)
//         .await
//         .expect("get_account")
//         .expect("greeted_account not found");
//     assert_eq!(
//         GreetingAccount::try_from_slice(&greeted_account.data)
//             .unwrap()
//             .counter,
//         1
//     );
// }





#[tokio::test]
async fn test_handle_new_event() {
    let program_id = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "predictchain", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    let eventpda = Pubkey::create_program_address(&["event".as_bytes()], &program_id).unwrap();

    program_test.add_account(
        eventpda,
        Account {
            lamports: 1614820,
            data: vec![0_u8; mem::size_of::<EventPDA>()],
            owner: program_id,
            ..Account::default()
        },
    );
    
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let rent = banks_client.get_rent().await.unwrap();
    // let eventpda_size = mem::size_of::<EventPDA>();
    // let eventpda_rent = rent.minimum_balance(eventpda_size);

    // program_test.add_account(
    //     eventpda,
    //     Account {
    //         lamports: eventpda_rent,
    //         data: vec![0_u8; eventpda_size],
    //         owner: program_id,
    //         ..Account::default()
    //     },
    // );


    // let rent = banks_client.get_rent().await.unwrap();
    // let eventpda_size = mem::size_of::<EventPDA>();
    // let eventpda_rent = rent.minimum_balance(eventpda_size);
    let mint_rent = rent.minimum_balance(spl_token::state::Mint::LEN);

    // let (eventpda, bump_dseed) = Pubkey::find_program_address(&["event".as_bytes()], &program_id);
    // let eventpda = Pubkey::create_program_address(&["event".as_bytes()], &program_id).unwrap();

    let yes_mint = Keypair::new();
    let no_mint = Keypair::new();

    // let mut transaction = Transaction::new_with_payer(
    //     &[
    //         system_instruction::create_account(
    //             &payer.pubkey(),
    //             &eventpda,
    //             eventpda_rent,
    //             eventpda_size as u64,
    //             &program_id,
    //         ),
    //     ],
    //     Some(&payer.pubkey()),
    // );

    // let authority_signature_seeds = [&["event".as_bytes()], &[&[bump_seed]]];
    // let signers = &[&authority_signature_seeds[..]];

    // transaction.sign(signers, recent_blockhash);
    // banks_client.process_transaction(transaction).await;

    // create mints
    let mut transaction1 = Transaction::new_with_payer(
        &[  
            system_instruction::create_account(
                &payer.pubkey(),
                &yes_mint.pubkey(),
                mint_rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            system_instruction::create_account(
                &payer.pubkey(),
                &no_mint.pubkey(),
                mint_rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &yes_mint.pubkey(),
                &program_id,
                None,
                0,
            )
            .unwrap(),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &no_mint.pubkey(),
                &program_id,
                None,
                0,
            )
            .unwrap(),

        ],
        Some(&payer.pubkey()),
    );
    transaction1.sign(&[&payer, &yes_mint, &no_mint], recent_blockhash);
    banks_client.process_transaction(transaction1).await.unwrap();


    let mut transaction2 = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0], // ignored but makes the instruction unique in the slot
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(eventpda, false),
                AccountMeta::new(yes_mint.pubkey(), false),
                AccountMeta::new(no_mint.pubkey(), false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction2.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction2).await.unwrap();

    let eventpda_account = banks_client
        .get_account(eventpda)
        .await
        .expect("get_account")
        .expect("event_pda not found");
    
    let event_data = EventPDA::try_from_slice(&eventpda_account.data).unwrap();

    assert_eq!(
        event_data.yes_mint_address,
        yes_mint.pubkey()
    );

    assert_eq!(
        event_data.no_mint_address,
        no_mint.pubkey()
    );

}



// #[tokio::test]
// async fn test_deposit_with_program_authority() {
//     let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

//     let deposit_amount = 100;

//     let pool = TestPool::new();

//     pool.init_pool(&mut banks_client, &payer, &recent_blockhash)
//         .await;

//     let user_account = Keypair::new();
//     let user_account_owner = Keypair::new();
//     let user_pass_account = Keypair::new();
//     let user_fail_account = Keypair::new();

//     pool.prepare_accounts_for_deposit(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         deposit_amount,
//         deposit_amount,
//         &user_account,
//         &pool.authority,
//         &user_account_owner,
//         &user_pass_account,
//         &user_fail_account,
//     )
//     .await;

//     let user_balance_before = get_token_balance(&mut banks_client, &user_account.pubkey()).await;
//     assert_eq!(user_balance_before, deposit_amount);

//     // Make deposit
//     pool.make_deposit(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &user_account,
//         &user_pass_account,
//         &user_fail_account,
//         deposit_amount,
//     )
//     .await;

//     // Check balance of user account
//     let user_balance_after = get_token_balance(&mut banks_client, &user_account.pubkey()).await;
//     assert_eq!(user_balance_after, 0);

//     // Check balance of pool deposit account
//     let pool_deposit_account_balance =
//         get_token_balance(&mut banks_client, &pool.pool_deposit_account.pubkey()).await;
//     assert_eq!(pool_deposit_account_balance, deposit_amount);

//     // Check if user has PASS and FAIL tokens
//     let user_pass_tokens = get_token_balance(&mut banks_client, &user_pass_account.pubkey()).await;
//     assert_eq!(user_pass_tokens, deposit_amount);

//     let user_fail_tokens = get_token_balance(&mut banks_client, &user_fail_account.pubkey()).await;
//     assert_eq!(user_fail_tokens, deposit_amount);
// }



// pub async fn create_account(
//     banks_client: &mut BanksClient,
//     payer: &Keypair,
//     recent_blockhash: &Hash,
//     account: &Keypair,
//     rent: u64,
//     space: u64,
//     owner: &Pubkey,
// ) -> Result<(), TransportError> {
//     let mut transaction = Transaction::new_with_payer(
//         &[system_instruction::create_account(
//             &payer.pubkey(),
//             &account.pubkey(),
//             rent,
//             space,
//             owner,
//         )],
//         Some(&payer.pubkey()),
//     );

//     let asdf = Keypair::new();

//     transaction.sign(&[payer, account, &asdf], *recent_blockhash);
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

// async fn get_account(banks_client: &mut BanksClient, pubkey: &Pubkey) -> Account {
//     banks_client
//         .get_account(*pubkey)
//         .await
//         .expect("account not found")
//         .expect("account empty")
// }

// pub async fn create_mint(
//     banks_client: &mut BanksClient,
//     payer: &Keypair,
//     recent_blockhash: &Hash,
//     mint_account: &Keypair,
//     mint_rent: u64,
//     owner: &Pubkey,
// ) -> Result<(), TransportError> {
//     let mut transaction = Transaction::new_with_payer(
//         &[
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &mint_account.pubkey(),
//                 mint_rent,
//                 spl_token::state::Mint::LEN as u64,
//                 &spl_token::id(),
//             ),
//             spl_token::instruction::initialize_mint(
//                 &spl_token::id(),
//                 &mint_account.pubkey(),
//                 &owner,
//                 None,
//                 0,
//             )
//             .unwrap(),
//         ],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer, mint_account], *recent_blockhash);
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

