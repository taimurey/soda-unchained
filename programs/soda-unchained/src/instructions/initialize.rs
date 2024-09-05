use account_compression::{
    utils::constants::CPI_AUTHORITY_PDA_SEED, AddressMerkleTreeConfig, AddressQueueConfig,
};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    token_2022::{initialize_account3, InitializeAccount3},
};
use light_compressed_token::program::LightCompressedToken;

#[derive(Accounts)]
pub struct ServerInitialize<'info> {
    /// creator account
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub token_pool_pda: Account<'info, TokenAccount>,

    /// mint account
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    /// CHECK: Soda Authority
    pub soda_authority: AccountInfo<'info>,

    /// CHECK:
    #[account(seeds = [b"cpi_authority"], bump)]
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
    pub light_system_program: Program<'info, Token>,

    pub compressed_token_program: Program<'info, LightCompressedToken>,

    /// CHECK:
    pub account_compression_authority: UncheckedAccount<'info>,

    /// CHECK: (different program) will be checked by the system program
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,

    /// account compression program
    pub account_compression_program: Program<'info, Token>,
}

/// Creates a merkle tree for each server and mints a token to server creator
/// Creator Address is inserted into merkle tree
pub fn initialize_server(
    ctx: Context<ServerInitialize>,
    merkle_tree_config: AddressMerkleTreeConfig,
    queue_config: AddressQueueConfig,
    bump: u8,
    index: u64,
    amount: u64,
) -> Result<()> {
    let bump = &[bump];
    let seeds = [CPI_AUTHORITY_PDA_SEED, bump];
    let signer_seeds = &[&seeds[..]];
    let accounts = account_compression::cpi::accounts::InitializeAddressMerkleTreeAndQueue {
        authority: ctx.accounts.cpi_authority_pda.to_account_info(),
        merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
        queue: ctx.accounts.queue.to_account_info(),
        registered_program_pda: Some(ctx.accounts.registered_program.clone()),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.account_compression_program.to_account_info(),
        accounts,
        signer_seeds,
    );

    account_compression::cpi::initialize_address_merkle_tree_and_queue(
        cpi_ctx,
        index,
        Some(ctx.accounts.soda_authority.key()),
        None,
        merkle_tree_config,
        queue_config,
    )?;

    // TODO: Emit Merkle Tree Configs

    let space = Mint::LEN;
    let lamports = Rent::get()?.minimum_balance(space);

    let cpi_accounts = system_program::CreateAccount {
        from: ctx.accounts.creator.to_account_info(),
        to: ctx.accounts.mint.to_account_info(),
    };

    let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);

    anchor_lang::system_program::create_account(
        cpi_context,
        lamports,
        space as u64,
        &ctx.accounts.creator.key(),
    )?;

    let cpi_accounts = InitializeAccount3 {
        account: ctx.accounts.mint.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.creator.to_account_info(),
    };

    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    initialize_account3(cpi_context)?;

    light_compressed_token::cpi::create_token_pool(ctx.accounts.set_token_pool_ctx())?;

    light_compressed_token::cpi::mint_to(
        ctx.accounts.set_mint_ctx(),
        vec![ctx.accounts.creator.key()],
        vec![amount],
        None,
    )?;

    // TODO : add dynamic support for addresses

    account_compression::cpi::insert_addresses(
        ctx.accounts.set_insert_addresses_ctx(),
        vec![ctx.accounts.creator.key().to_bytes()],
    )?;

    Ok(())
}
