use std::rc::Rc;

use crate::load_cfg;
use account_compression::{AddressMerkleTreeConfig, AddressQueueConfig};
use anchor_client::solana_client::nonblocking::rpc_client::RpcClient;
use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use anchor_client::{Client, Cluster};
use anchor_spl::token::spl_token;
use light_compressed_token::get_token_pool_pda;
use light_sdk::utils::get_cpi_authority_pda;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};

pub async fn test_initialize_server() -> Result<(), Box<dyn std::error::Error>> {
    let client_config = "client_config.ini";
    let pool_config = load_cfg(&client_config.to_string()).unwrap();

    let payer = read_keypair_file(&pool_config.payer_path)?;
    let rpc_client = RpcClient::new(pool_config.http_url.to_string());

    // anchor client.
    let anchor_config = pool_config.clone();
    let url = Cluster::Custom(anchor_config.http_url, anchor_config.ws_url);
    let wallet = read_keypair_file(&pool_config.payer_path)?;
    let anchor_client = Client::new(url, Rc::new(wallet));
    let program = anchor_client.program(pool_config.soda_unchained_program)?;

    // Create necessary keypairs
    let creator = Keypair::new();
    let mint = Keypair::new();

    let token_pool_pda = get_token_pool_pda(&mint.pubkey());
    let cpi_authority_pda = get_cpi_authority_pda(&program.id());
    let (soda_authority, _) = Pubkey::find_program_address(&[b"soda_authority"], &program.id());
    let (registered_program, _) =
        Pubkey::find_program_address(&[b"registered_program"], &account_compression::id());

    let merkle_tree = Keypair::new();
    let queue = Keypair::new();

    // Create the instruction
    let merkle_tree_config = AddressMerkleTreeConfig::default();
    let queue_config = AddressQueueConfig::default();
    let amount = 1;

    let ix = program
        .request()
        .accounts(soda_unchained::accounts::ServerInitialize {
            creator: creator.pubkey(),
            token_pool_pda: token_pool_pda,
            mint: mint.pubkey(),
            soda_authority,
            cpi_authority_pda,
            registered_program,
            noop_program: Pubkey::new_from_array(
                account_compression::utils::constants::NOOP_PUBKEY,
            ),

            system_program: solana_sdk::system_program::id(),
            token_program: spl_token::id(),
            queue: queue.pubkey(),
            light_system_program: light_system_program::id(),
            compressed_token_program: light_compressed_token::id(),
            account_compression_authority: account_compression::id(),
            merkle_tree: merkle_tree.pubkey(),
            account_compression_program: account_compression::id(),
        })
        .args(soda_unchained::instruction::InitializeServer {
            merkle_tree_config: merkle_tree_config,
            queue_config: queue_config,
            bump: 0,
            index: 0,
            amount,
        })
        .send_with_spinner_and_config(RpcSendTransactionConfig {
            ..Default::default()
        });

    Ok(())
}
