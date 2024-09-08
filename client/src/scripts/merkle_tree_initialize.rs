use account_compression::sdk::create_initialize_address_merkle_tree_and_queue_instruction;
use account_compression::{
    AddressMerkleTreeAccount, AddressMerkleTreeConfig, AddressQueueConfig, QueueAccount,
};
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use colored::Colorize;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::Instruction;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};

use crate::utils::config::load_cfg;
use crate::utils::transaction_link;

pub fn initialize_merkle() -> Result<(), Box<dyn std::error::Error>> {
    let client_config = "client_config.ini";
    let pool_config = load_cfg(&client_config.to_string()).unwrap();

    println!("{}", "Creating new Merkle Tree".bold().white());

    let payer = read_keypair_file(&pool_config.payer_path)?;
    let rpc_client = RpcClient::new(pool_config.http_url.to_string());

    // anchor client.
    // let anchor_config = pool_config.clone();
    // let url = Cluster::Custom(anchor_config.http_url, anchor_config.ws_url);
    // let wallet = read_keypair_file(&pool_config.payer_path)?;
    // let anchor_client = Client::new(url, Rc::new(wallet));
    // let program = anchor_client.program(pool_config.soda_unchained_program)?;

    // let mint = Keypair::new();

    let merkle_tree_keypair = Keypair::new();
    let queue_keypair = Keypair::new();

    // Create the instruction
    let merkle_tree_config = AddressMerkleTreeConfig::default();
    let queue_config = AddressQueueConfig::default();

    let queue_size =
        QueueAccount::size(account_compression::utils::constants::ADDRESS_QUEUE_VALUES as usize)
            .unwrap();

    // let remaining_accounts = RemainingAccounts::default();

    // let account = new_compressed_account(
    //     payer,
    //     address_seed,
    //     program_id,
    //     merkle_output_context,
    //     address_merkle_context,
    //     address_merkle_tree_root_index,
    //     remaining_accounts,
    // );

    let tree_size = AddressMerkleTreeAccount::size(
        merkle_tree_config.height as usize,
        merkle_tree_config.changelog_size as usize,
        merkle_tree_config.roots_size as usize,
        merkle_tree_config.canopy_depth as usize,
        merkle_tree_config.address_changelog_size as usize,
    );

    let queue_account_create_ix = create_account_instruction(
        &payer.pubkey(),
        queue_size,
        rpc_client
            .get_minimum_balance_for_rent_exemption(queue_size)
            .unwrap(),
        &account_compression::id(),
        Some(&queue_keypair),
    );
    let mt_account_create_ix = create_account_instruction(
        &payer.pubkey(),
        tree_size,
        rpc_client
            .get_minimum_balance_for_rent_exemption(tree_size)
            .unwrap(),
        &account_compression::id(),
        Some(&merkle_tree_keypair),
    );

    let merkle_tree = create_initialize_address_merkle_tree_and_queue_instruction(
        0,
        payer.pubkey(),
        None,
        None,
        Some(Pubkey::new_unique()),
        merkle_tree_keypair.pubkey(),
        queue_keypair.pubkey(),
        merkle_tree_config,
        queue_config,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[queue_account_create_ix, mt_account_create_ix, merkle_tree],
        Some(&payer.pubkey()),
        &vec![&payer, &queue_keypair, &merkle_tree_keypair],
        rpc_client.get_latest_blockhash().unwrap(),
    );

    let signature = rpc_client.send_transaction_with_config(
        &transaction,
        RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        },
    )?;

    transaction_link(signature);

    rpc_client.confirm_transaction_with_spinner(
        &signature,
        &rpc_client.get_latest_blockhash()?,
        CommitmentConfig {
            commitment: solana_sdk::commitment_config::CommitmentLevel::Confirmed,
        },
    )?;

    Ok(())
}

pub fn create_account_instruction(
    payer: &Pubkey,
    size: usize,
    rent: u64,
    id: &Pubkey,
    keypair: Option<&Keypair>,
) -> Instruction {
    let keypair = match keypair {
        Some(keypair) => keypair.insecure_clone(),
        None => Keypair::new(),
    };
    system_instruction::create_account(payer, &keypair.pubkey(), rent, size as u64, id)
}
