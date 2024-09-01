use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid signer")]
    InvalidSigner,
    #[msg("Invalid Param")]
    InvalidParam,
}
