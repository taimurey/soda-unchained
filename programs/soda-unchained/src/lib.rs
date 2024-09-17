pub mod error;
pub mod instructions;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;
use light_sdk::{
    merkle_context::{PackedAddressMerkleContext, PackedMerkleOutputContext},
    utils::CompressedProof,
};
use light_system_program::sdk::CompressedCpiContext;

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

    /// membership token
    pub fn membership_token(ctx: Context<MembershipMint>, param: u8) -> Result<()> {
        instructions::manage_membership(ctx, param)
    }

    /// store compressed accounts
    #[allow(clippy::too_many_arguments)]
    pub fn store_compressed_tokens<'info>(
        ctx: Context<'_, '_, '_, 'info, AirdropInfo<'info>>,
        proof: CompressedProof,
        merkle_output_context: PackedMerkleOutputContext,
        address_merkle_context: PackedAddressMerkleContext,
        address_merkle_tree_root_index: u16,
        account_name: String,
        rdata: RData,
        cpi_context: Option<CompressedCpiContext>,
    ) -> Result<()> {
        instructions::store_accounts(
            ctx,
            proof,
            merkle_output_context,
            address_merkle_context,
            address_merkle_tree_root_index,
            account_name,
            rdata,
            cpi_context,
        )
    }
}
