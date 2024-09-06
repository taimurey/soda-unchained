use std::str::FromStr;

use configparser::ini::Ini;
use solana_sdk::pubkey::Pubkey;
pub mod intiailize_test;

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientConfig {
    http_url: String,
    ws_url: String,
    payer_path: String,
    admin_path: String,
    soda_unchained_program: Pubkey,
}

pub fn load_cfg(client_config: &String) -> eyre::Result<ClientConfig> {
    let mut config = Ini::new();
    let _map = config.load(client_config).unwrap();
    let http_url = config.get("Global", "http_url").unwrap();
    if http_url.is_empty() {
        panic!("http_url must not be empty");
    }
    let ws_url = config.get("Global", "ws_url").unwrap();
    if ws_url.is_empty() {
        panic!("ws_url must not be empty");
    }
    let payer_path = config.get("Global", "payer_path").unwrap();
    if payer_path.is_empty() {
        panic!("payer_path must not be empty");
    }
    let admin_path = config.get("Global", "admin_path").unwrap();
    if admin_path.is_empty() {
        panic!("admin_path must not be empty");
    }

    let raydium_cp_program_str = config.get("Global", "soda_unchained_program").unwrap();
    if raydium_cp_program_str.is_empty() {
        panic!("soda_unchained_program must not be empty");
    }
    let soda_unchained_program = Pubkey::from_str(&raydium_cp_program_str).unwrap();

    Ok(ClientConfig {
        http_url,
        ws_url,
        payer_path,
        admin_path,
        soda_unchained_program,
    })
}
