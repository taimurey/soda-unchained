pub mod config;
pub mod log;

use ::log::info;
use solana_sdk::signature::Signature;

pub fn transaction_link(link: Signature) {
    info!(
        "Signature: \nhttps://solscan.io/tx/{}?cluster=custom&customUrl=\
    https://devnet.helius-rpc.com/?api-key=16ef3f61-7567-47d9-9c44-edec13422455",
        link
    );
}

pub fn clear() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
