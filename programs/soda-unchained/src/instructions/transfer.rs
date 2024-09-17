use std::net::{Ipv4Addr, Ipv6Addr};

use account_compression::{program::AccountCompression, RegisteredProgram};
use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use light_hasher::bytes::AsByteVec;
use light_sdk::{
    light_account,
    merkle_context::{PackedAddressMerkleContext, PackedMerkleOutputContext},
    program_merkle_context::unpack_address_merkle_context,
    utils::create_cpi_inputs_for_new_account,
    verify::verify,
    LightTraits,
};
use light_system_program::{
    invoke::processor::CompressedProof, invoke_cpi::account::CpiContextAccount,
    program::LightSystemProgram, sdk::CompressedCpiContext,
};

#[derive(Accounts, LightTraits)]
pub struct AirdropInfo<'info> {
    #[account(mut)]
    #[fee_payer]
    pub signer: Signer<'info>,
    #[self_program]
    pub self_program: Program<'info, crate::program::SodaUnchained>,
    /// CHECK: Checked in light-system-program.
    #[authority]
    pub cpi_signer: AccountInfo<'info>,
    #[cpi_context]
    pub cpi_context_account: Account<'info, CpiContextAccount>,
    pub light_system_program: Program<'info, LightSystemProgram>,
    pub registered_program_pda: Account<'info, RegisteredProgram>,
    /// CHECK: Pass Noop_program
    pub noop_program: AccountInfo<'info>,
    /// CHECK:
    pub account_compression_authority: AccountInfo<'info>,
    pub account_compression_program: Program<'info, AccountCompression>,
    /// CHECK:
    pub system_program: Program<'info, System>,
}

#[light_account]
#[derive(Debug)]
pub struct NameRecord {
    #[truncate]
    pub owner: Pubkey,
    #[truncate]
    pub name: String,
    pub rdata: RData,
}

#[derive(Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum RData {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CName(String),
}

impl anchor_lang::IdlBuild for RData {}
impl AsByteVec for RData {
    fn as_byte_vec(&self) -> Vec<Vec<u8>> {
        match self {
            Self::A(ipv4_addr) => vec![ipv4_addr.octets().to_vec()],
            Self::AAAA(ipv6_addr) => vec![ipv6_addr.octets().to_vec()],
            Self::CName(cname) => cname.as_byte_vec(),
        }
    }
}

pub fn store_accounts<'info>(
    ctx: Context<'_, '_, '_, 'info, AirdropInfo<'info>>,
    proof: CompressedProof,
    merkle_output_context: PackedMerkleOutputContext,
    address_merkle_context: PackedAddressMerkleContext,
    address_merkle_tree_root_index: u16,
    account_name: String,
    rdata: RData,
    cpi_context: Option<CompressedCpiContext>,
) -> Result<()> {
    use light_sdk::{address::derive_address_seed, compressed_account::new_compressed_account};

    let unpacked_address_merkle_context =
        unpack_address_merkle_context(address_merkle_context, ctx.remaining_accounts);

    let address_seed = derive_address_seed(
        &[
            ctx.accounts.signer.key().to_bytes().as_slice(),
            account_name.as_bytes(),
        ],
        &crate::ID,
        &unpacked_address_merkle_context,
    );

    let record = NameRecord {
        owner: ctx.accounts.signer.key(),
        name: account_name,
        rdata,
    };

    let (compressed_account, new_address_params) = new_compressed_account(
        &record,
        &address_seed,
        &crate::ID,
        &merkle_output_context,
        &address_merkle_context,
        address_merkle_tree_root_index,
        ctx.remaining_accounts,
    )?;

    let signer_seed = b"cpi_signer".as_slice();
    let bump = Pubkey::find_program_address(&[signer_seed], ctx.accounts.self_program.key).1;
    let signer_seeds = [signer_seed, &[bump]];

    let inputs = create_cpi_inputs_for_new_account(
        proof,
        new_address_params,
        compressed_account,
        cpi_context,
    );

    verify(&ctx, &inputs, &[&signer_seeds])?;

    Ok(())
}
