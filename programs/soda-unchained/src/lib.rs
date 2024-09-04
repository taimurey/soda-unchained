pub mod error;
pub mod instructions;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;

#[cfg(not(feature = "devnet"))]
declare_id!("DJEXkPy4z1K9g8RvGUVqHy3EivJU64drXv3FvTJSJgbw");

#[cfg(feature = "devnet")]
declare_id!("DJEXkPy4z1K9g8RvGUVqHy3EivJU64drXv3FvTJSJgbw");

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

    // /// Initialize the server
    // pub fn initialize_server(ctx: Context<ServerInit>) -> Result<()> {
    //     instructions::initialize::initialize_server(ctx, 1)
    // }

    // /// membership token
    // pub fn membership_token(ctx: Context<MembershipMint>, param: u8) -> Result<()> {
    //     instructions::manage_membership(ctx, param)
    // }
}
