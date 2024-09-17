use account_compression::program::AccountCompression;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use light_compressed_token::program::LightCompressedToken;
use light_system_program::program::LightSystemProgram;

use crate::utils::state::assert_accounts;

#[derive(Accounts)]
pub struct ServerInitialize<'info> {
    /// creator account
    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub token_pool_pda: AccountInfo<'info>,

    // mint address
    #[account(
        init,
        payer = creator,
        mint::authority = creator,
        mint::decimals = 0,
        mint::freeze_authority = creator,
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: Soda Authority
    pub soda_authority: AccountInfo<'info>,

    /// CHECK:
    pub cpi_authority_pda: AccountInfo<'info>,
    /// CHECK: registered program
    pub registered_program: AccountInfo<'info>,

    /// CHECK: noop program
    pub noop_program: AccountInfo<'info>,
    /// System-Program
    pub system_program: Program<'info, System>,

    /// Token-Program or Token-Program
    pub token_program: Program<'info, Token>,
    /// CHECK:
    #[account(mut)]
    pub queue: AccountInfo<'info>,

    /// Light-Compressed-Token
    pub light_system_program: Program<'info, LightSystemProgram>,

    pub compressed_token_program: Program<'info, LightCompressedToken>,

    /// CHECK:
    pub account_compression_authority: AccountInfo<'info>,

    /// CHECK: (different program) will be checked by the system program
    #[account(mut
        // init,
        // space = 8 + std::mem::size_of::<AddressMerkleTreeAccount>(),
        // payer = creator,
        // owner = account_compression_program.key()
    )]
    pub merkle_tree: AccountInfo<'info>,

    /// account compression program
    pub account_compression_program: Program<'info, AccountCompression>,
}

/// Creates a merkle tree for each server and mints a token to server creator
/// Creator Address is inserted into merkle tree
pub fn initialize_server(ctx: Context<ServerInitialize>, amount: u64) -> Result<()> {
    light_compressed_token::cpi::create_token_pool(ctx.accounts.set_token_pool_ctx())?;

    assert_accounts(ctx.accounts)?;

    light_compressed_token::cpi::mint_to(
        ctx.accounts.set_mint_ctx(),
        vec![ctx.accounts.creator.key()],
        vec![amount],
        None,
    )?;

    // light_compressed_token::cpi::transfer(ctx.accounts.set_transfer_ctx(), inputs)?;

    // account_compression::cpi::insert_addresses(
    //     ctx.accounts.set_insert_addresses_ctx(),
    //     vec![ctx.accounts.creator.key().to_bytes()],
    // )?;

    Ok(())
}
