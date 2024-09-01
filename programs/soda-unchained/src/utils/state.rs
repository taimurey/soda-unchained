use anchor_lang::prelude::Pubkey;
use core::mem::size_of;

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompressedMintToProps {
    pub public_keys: Vec<Pubkey>,
    pub amounts: Vec<u64>,
    pub lamports: Option<u64>,
}

impl CompressedMintToProps {
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());

        buf.extend_from_slice(&self.public_keys.len().to_le_bytes());
        for key in &self.public_keys {
            buf.extend_from_slice(key.as_ref());
        }

        buf.extend_from_slice(&self.amounts.len().to_le_bytes());
        buf
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServerState {
    /// index
    pub index: u8,

    /// compressed mint address of the server
    pub mint: Pubkey,
}

impl ServerState {
    pub const LEN: usize = 1 + 32;
}
