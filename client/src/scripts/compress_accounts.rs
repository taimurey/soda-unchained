use account_compression::{AddressMerkleTreeConfig, AddressQueueConfig, QueueAccount};
use anchor_client::solana_client::rpc_client::RpcClient;
use colored::Colorize;
use light_sdk::address::derive_address_seed;
use light_sdk::merkle_context::{AddressMerkleContext, RemainingAccounts};
use light_system_program::sdk::address::derive_address;
use light_test_utils::indexer::{Indexer, TestIndexer};
use light_test_utils::rpc::ProgramTestRpcConnection;
use light_test_utils::test_env::{setup_test_programs_with_accounts, EnvAccounts};

use solana_sdk::signature::{read_keypair_file, Keypair};

use crate::utils::config::load_cfg;

pub async fn compress_accounts() -> Result<(), Box<dyn std::error::Error>> {
    let client_config = "client_config.ini";
    let pool_config = load_cfg(&client_config.to_string()).unwrap();

    println!("{}", "Creating new Merkle Tree".bold().white());

    let payer = read_keypair_file(&pool_config.payer_path)?;
    let rpc_client = RpcClient::new(pool_config.http_url.to_string());

    rpc_client
        .send_and_confirm_transaction_with_spinner(transaction)
        .await?;

    let (mut rpc, env) = setup_test_programs_with_accounts(None).await;
    let env = EnvAccounts::get_local_test_validator_accounts();

    let mut remaining_accounts = RemainingAccounts::default();

    let address_merkle_context = AddressMerkleContext {
        address_merkle_tree_pubkey: env.address_merkle_tree_pubkey,
        address_queue_pubkey: env.address_merkle_tree_queue_pubkey,
    };

    let mut test_indexer: TestIndexer<ProgramTestRpcConnection> =
        TestIndexer::init_from_env(&payer, &env, true, true).await;

    let name = "example.io";

    let address_seed = derive_address_seed(
        &[b"name-service", name.as_bytes()],
        &soda_unchained::ID,
        &address_merkle_context,
    );

    println!("ADDRESS_SEED: {address_seed:?}");

    let address = derive_address(&env.address_merkle_tree_pubkey, &address_seed).unwrap();

    println!("ADDRESS_SEED: {address:?}");

    let rpc_result = test_indexer
        .create_proof_for_compressed_accounts(
            None,
            None,
            Some(&[address]),
            Some(vec![env.address_merkle_tree_pubkey]),
            &mut rpc,
        )
        .await;

    println!("{:#?}", rpc_result);

    Ok(())
}
