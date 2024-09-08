pub mod error;
pub mod instructions;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;

#[cfg(not(feature = "devnet"))]
declare_id!("5cqC671u7TqttSQC3MBMTj41KkWC8pTebcg1kbXwP1hZ");

#[cfg(feature = "devnet")]
declare_id!("5cqC671u7TqttSQC3MBMTj41KkWC8pTebcg1kbXwP1hZ");

pub mod admin {
    use anchor_lang::prelude::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("Bn6jUQPC48meSkE5nZ8G8yWyxsuoiGwQwyX127nVmWWZ");
    #[cfg(not(feature = "devnet"))]
    declare_id!("Bn6jUQPC48meSkE5nZ8G8yWyxsuoiGwQwyX127nVmWWZ");
}

#[program]
pub mod soda_unchained {
    use super::*;

    /// Initialize the server
    pub fn initialize_server(ctx: Context<ServerInitialize>, amount: u64) -> Result<()> {
        instructions::initialize::initialize_server(ctx, amount)
    }

    // /// membership token
    // pub fn membership_token(ctx: Context<MembershipMint>, param: u8) -> Result<()> {
    //     instructions::manage_membership(ctx, param)
    // }
}
