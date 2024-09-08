use crate::ServerInitialize;
use anchor_lang::prelude::*;

// #[repr(C)]
// #[derive(Clone, Debug, Default, PartialEq)]
// pub struct ServerState {
//     /// index
//     pub index: u8,

//     /// compressed mint address of the server
//     pub mint: Pubkey,
// }

// impl ServerState {
//     pub const LEN: usize = 8 + 1 + 32 + 32;
// }

pub fn assert_accounts(accounts: &mut ServerInitialize) -> Result<()> {
    msg!("Checking token_pool_pda");
    let _ = accounts.token_pool_pda.to_account_info();

    msg!("Checking registered_program");
    let _ = accounts.registered_program.to_account_info();

    msg!("Checking queue");
    let _ = accounts.queue.to_account_info();

    Ok(())
}
