use std::rc::Rc;
use std::str::FromStr;

use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use anchor_client::{Client, Cluster};
use anchor_spl::token::spl_token;
use colored::Colorize;
use light_compressed_token::get_token_pool_pda;
use light_compressed_token::process_transfer::get_cpi_authority_pda;
use photon_api::apis::configuration::{ApiKey, Configuration};
use photon_api::apis::default_api::get_compressed_token_accounts_by_owner_post;
use photon_api::models::{
    GetCompressedTokenAccountsByOwnerPostRequest,
    GetCompressedTokenAccountsByOwnerPostRequestParams, TokenAcccount, TokenAccountList,
};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};

use crate::utils::{config::load_cfg, transaction_link};

pub async fn test_initialize_server() -> Result<(), Box<dyn std::error::Error>> {
    let client_config = "client_config.ini";
    let pool_config = load_cfg(&client_config.to_string()).unwrap();

    println!(
        "Invoking Initialize in {}",
        pool_config
            .soda_unchained_program
            .to_string()
            .bold()
            .green(),
    );

    let payer = read_keypair_file(&pool_config.payer_path)?;
    let rpc_client = RpcClient::new(pool_config.http_url.to_string());

    // anchor client.
    let anchor_config = pool_config.clone();
    let url = Cluster::Custom(anchor_config.http_url, anchor_config.ws_url);
    let wallet = read_keypair_file(&pool_config.payer_path)?;
    let anchor_client = Client::new(url, Rc::new(wallet));
    let program = anchor_client.program(pool_config.soda_unchained_program)?;

    let mint = Keypair::new();

    let registered_program_pda = Pubkey::find_program_address(
        &[light_system_program::ID.to_bytes().as_slice()],
        &account_compression::ID,
    )
    .0;
    let account_compression_authority =
        light_system_program::utils::get_cpi_authority_pda(&light_system_program::ID);
    let token_pool_pda = get_token_pool_pda(&mint.pubkey());
    let (cpi_authority_pda, _) = get_cpi_authority_pda();
    let (soda_authority, _) = Pubkey::find_program_address(&[b"soda_authority"], &program.id());

    let merkle_tree = Pubkey::from_str("smt1NamzXdq4AMqS2fS2F1i5KTYPZRhoHgWx38d8WsT")?;
    //Keypair::from_base58_string(&pool_config.merkle_keypair);
    let queue = Keypair::from_base58_string(&pool_config.queue_keypair);

    // Create the instruction
    // let merkle_tree_config = AddressMerkleTreeConfig::default();
    // let queue_config = AddressQueueConfig::default();
    let amount = 1;

    let ix = program
        .request()
        .accounts(soda_unchained::accounts::ServerInitialize {
            creator: payer.pubkey(),
            token_pool_pda: token_pool_pda,
            mint: mint.pubkey(),
            soda_authority,
            cpi_authority_pda,
            registered_program: registered_program_pda,
            noop_program: Pubkey::new_from_array(
                account_compression::utils::constants::NOOP_PUBKEY,
            ),
            system_program: solana_sdk::system_program::id(),
            token_program: spl_token::id(),
            queue: queue.pubkey(),
            light_system_program: light_system_program::id(),
            compressed_token_program: light_compressed_token::id(),
            account_compression_authority: account_compression_authority,
            merkle_tree: merkle_tree,
            account_compression_program: account_compression::id(),
        })
        .args(soda_unchained::instruction::InitializeServer { amount })
        .instructions()?;

    let signers = vec![&payer, &mint];
    let recent_hash = rpc_client.get_latest_blockhash()?;
    let txn = Transaction::new_signed_with_payer(&ix, Some(&payer.pubkey()), &signers, recent_hash);

    let signature = rpc_client.send_transaction_with_config(
        &txn,
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

    println!("\n\n{}\n", "Test Passed...".italic().green());

    let mut photon = Configuration::new();
    photon.base_path = "https://devnet.helius-rpc.com".to_string();
    photon.api_key = Some(ApiKey {
        prefix: None,
        key: "16ef3f61-7567-47d9-9c44-edec13422455".to_string(),
    });

    let compressed_accounts = get_compressed_token_accounts_by_owner_post(
        &photon,
        GetCompressedTokenAccountsByOwnerPostRequest {
            params: Box::new(GetCompressedTokenAccountsByOwnerPostRequestParams {
                mint: Some(Some(mint.pubkey().to_string())),
                owner: payer.pubkey().to_string(),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .await?;

    let accounts = select_min_compressed_token_accounts_for_transfer(
        &mut compressed_accounts.result.clone().unwrap().value,
        amount,
    )?;

    println!("{:#?}", accounts);

    // let proof = get_validity_proof_post(
    //     &photon,
    //     GetValidityProofPostRequest {
    //         params: Box::new(GetValidityProofPostRequestParams {
    //             hashes: compressed_accounts
    //                 .result
    //                 .clone()
    //                 .unwrap()
    //                 .value
    //                 .items
    //                 .iter()
    //                 .map(|token_account| token_account.account.hash.clone())
    //                 .collect::<Vec<_>>(),
    //             ..Default::default()
    //         }),
    //         ..Default::default()
    //     },
    // )
    // .await?;

    // println!("Proof: {:#?}", proof);

    Ok(())
}

pub fn send_txn(
    client: &RpcClient,
    txn: &Transaction,
    wait_confirm: bool,
) -> eyre::Result<Signature> {
    Ok(client.send_and_confirm_transaction_with_spinner_and_config(
        txn,
        if wait_confirm {
            CommitmentConfig::confirmed()
        } else {
            CommitmentConfig::processed()
        },
        RpcSendTransactionConfig {
            skip_preflight: true,
            ..RpcSendTransactionConfig::default()
        },
    )?)
}

fn select_min_compressed_token_accounts_for_transfer(
    account_list: &mut TokenAccountList,
    transfer_amount: u64,
) -> Result<(Vec<TokenAcccount>, u64, Option<u64>), String> {
    let mut accumulated_amount: u64 = 0;
    let mut accumulated_lamports: u64 = 0;
    let mut selected_accounts: Vec<TokenAcccount> = Vec::new();

    // Sort accounts in descending order based on lamports
    account_list
        .items
        .sort_by(|a, b| b.token_data.amount.cmp(&a.token_data.amount));

    for account in &account_list.items {
        if accumulated_amount >= transfer_amount {
            break;
        }
        accumulated_amount += account.token_data.amount as u64;
        accumulated_lamports += account.account.lamports as u64;
        selected_accounts.push(account.clone());
    }

    if accumulated_amount < transfer_amount {
        return Err(format!(
            "Not enough balance for transfer. Required: {}, available: {}",
            transfer_amount, accumulated_amount
        ));
    }

    let total_lamports = if accumulated_lamports < LAMPORTS_PER_SOL {
        Some(accumulated_lamports)
    } else {
        None
    };

    Ok((selected_accounts, accumulated_amount, total_lamports))
}
