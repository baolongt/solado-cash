use anchor_lang::prelude::*;

#[error_code]
pub enum AppError {
    #[msg("not owner")]
    NotOwner,

    #[msg("invalid amount")]
    InvalidAmount,

    #[msg("Invalid leaf index")]
    InvalidLeafIndex,

    #[msg("Invalid authority")]
    InvalidAuthority,
}
