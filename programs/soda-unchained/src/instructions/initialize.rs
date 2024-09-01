use anchor_lang::{
    prelude::*,
    solana_program::{self, instruction::Instruction},
    system_program,
};
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    token_2022::{initialize_account3, InitializeAccount3},
};

#[derive(Accounts)]
pub struct ServerInit<'info> {
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

    /// Light-Compressed-Token
    pub light_system_program: Program<'info, Token>,
    pub compressed_token_program: Program<'info, Token>,

    /// CHECK:
    pub account_compression_authority: UncheckedAccount<'info>,

    /// CHECK: (different program) will be checked by the system program
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    /// account compression program
    pub account_compression_program: Program<'info, Token>,
}

pub fn initialize_server(ctx: Context<ServerInit>, amount: u64) -> Result<()> {
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

    let cpi_accounts = vec![
        ctx.accounts.creator.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.token_pool_pda.to_account_info(),
        ctx.accounts.cpi_authority_pda.to_account_info(),
    ];

    let instruction = Instruction {
        program_id: ctx.accounts.compressed_token_program.key(),
        accounts: vec![
            AccountMeta::new(ctx.accounts.creator.key(), true),
            AccountMeta::new(ctx.accounts.mint.key(), false),
            AccountMeta::new(ctx.accounts.system_program.key(), false),
            AccountMeta::new(ctx.accounts.token_program.key(), false),
            AccountMeta::new(ctx.accounts.token_pool_pda.key(), false),
            AccountMeta::new(ctx.accounts.cpi_authority_pda.key(), false),
        ],
        data: vec![],
    };

    solana_program::program::invoke(&instruction, &cpi_accounts)?;

    light_compressed_token::cpi::mint_to(
        ctx.accounts.set_mint_ctx(),
        vec![ctx.accounts.creator.key()],
        vec![amount],
        None,
    )?;

    Ok(())
}
