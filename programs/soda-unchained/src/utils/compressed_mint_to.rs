use anchor_lang::prelude::*;
use light_compressed_token::cpi::accounts::MintToInstruction;

use crate::ServerInit;

impl<'info> ServerInit<'info> {
    pub fn set_mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintToInstruction<'info>> {
        let cpi_accounts = MintToInstruction {
            fee_payer: self.creator.to_account_info(),
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
}
