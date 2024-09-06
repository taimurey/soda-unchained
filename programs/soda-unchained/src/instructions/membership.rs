use account_compression::cpi::accounts::InsertIntoQueues;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use light_compressed_token::cpi::accounts::{BurnInstruction, MintToInstruction};

use crate::error::ErrorCode;

pub const POOL_SEED: &[u8] = b"pool";

#[derive(Accounts)]
pub struct MembershipMint<'info> {
    /// payer account
    #[account(mut)]
    pub payer: Signer<'info>,

    /// mint account
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    /// mint
    // TODO : Implement init in the function
    #[account(mut)]
    pub token_pool_pda: Account<'info, TokenAccount>,

    /// CHECK: Soda Authority
    pub soda_authority: AccountInfo<'info>,

    /// CHECK:
    #[account(seeds = [b"cpi_authority"], bump)]
    pub cpi_authority_pda: AccountInfo<'info>,

    /// CHECK: registered program
    pub registered_program: AccountInfo<'info>,

    /// CHECK: noop program
    pub noop_program: AccountInfo<'info>,

    /// acc

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
    pub merkle_tree: AccountInfo<'info>,

    /// account compression program
    pub account_compression_program: Program<'info, Token>,
}

impl<'info> MembershipMint<'info> {
    pub const LEN: usize = 15 * 32 + 15 * 32;

    pub fn set_mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintToInstruction<'info>> {
        let cpi_accounts = MintToInstruction {
            fee_payer: self.payer.to_account_info(),
            authority: self.soda_authority.to_account_info(),
            cpi_authority_pda: self.cpi_authority_pda.to_account_info(),
            mint: self.mint.to_account_info(),
            token_pool_pda: self.token_pool_pda.to_account_info(),
            token_program: self.token_program.to_account_info(),
            light_system_program: self.light_system_program.to_account_info(),
            registered_program_pda: self.registered_program.to_account_info(),
            noop_program: self.noop_program.to_account_info(),
            account_compression_authority: self.account_compression_authority.to_account_info(),
            account_compression_program: self.account_compression_program.to_account_info(),
            merkle_tree: self.merkle_tree.to_account_info(),
            self_program: self.compressed_token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            sol_pool_pda: None,
        };

        CpiContext::new(
            self.compressed_token_program.to_account_info(),
            cpi_accounts,
        )
    }

    pub fn set_burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, BurnInstruction<'info>> {
        let cpi_accounts = BurnInstruction {
            fee_payer: self.payer.to_account_info(),
            authority: self.soda_authority.to_account_info(),
            cpi_authority_pda: self.cpi_authority_pda.to_account_info(),
            mint: self.mint.to_account_info(),
            token_pool_pda: self.token_pool_pda.to_account_info(),
            token_program: self.token_program.to_account_info(),
            light_system_program: self.light_system_program.to_account_info(),
            registered_program_pda: self.registered_program.to_account_info(),
            noop_program: self.noop_program.to_account_info(),
            account_compression_authority: self.account_compression_authority.to_account_info(),
            account_compression_program: self.account_compression_program.to_account_info(),
            self_program: self.compressed_token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        CpiContext::new(
            self.compressed_token_program.to_account_info(),
            cpi_accounts,
        )
    }

    pub fn set_insert_addresses_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, InsertIntoQueues<'info>> {
        let cpi_accounts = InsertIntoQueues {
            fee_payer: self.payer.to_account_info(),
            authority: self.soda_authority.to_account_info(),
            registered_program_pda: Some(self.registered_program.to_account_info()),
            system_program: self.system_program.to_account_info(),
        };

        CpiContext::new(
            self.account_compression_program.to_account_info(),
            cpi_accounts,
        )
    }
}

pub fn manage_membership(ctx: Context<MembershipMint>, param: u8) -> Result<()> {
    match param {
        0 => {
            light_compressed_token::cpi::mint_to(
                ctx.accounts.set_mint_ctx(),
                vec![ctx.accounts.payer.key()],
                vec![1],
                None,
            )?;

            // TODO: merkle_tree size check
            // TODO: if true: rollover queue
            account_compression::cpi::insert_addresses(
                ctx.accounts.set_insert_addresses_ctx(),
                vec![ctx.accounts.payer.key().to_bytes()],
            )
        }
        // TODO : Build inputs to burn compressed mint token minted to the user
        // 1 => light_compressed_token::cpi::burn(ctx.accounts.set_burn_ctx(), inputs)
        _ => Err(ErrorCode::InvalidParam.into()),
    }
}
