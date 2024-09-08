use configparser::ini::Ini;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

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

    let merkle_keypair = config.get("Global", "merkle_keypair").unwrap();
    if merkle_keypair.is_empty() {
        panic!("payer_path must not be empty");
    }

    let queue_keypair = config.get("Global", "queue_keypair").unwrap();
    if queue_keypair.is_empty() {
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
        queue_keypair,
        merkle_keypair,
        admin_path,
        soda_unchained_program,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientConfig {
    pub http_url: String,
    pub ws_url: String,
    pub payer_path: String,
    pub admin_path: String,
    pub queue_keypair: String,
    pub merkle_keypair: String,
    pub soda_unchained_program: Pubkey,
}
