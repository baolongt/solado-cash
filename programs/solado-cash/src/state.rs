use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, Mint, TokenAccount };

#[account]
#[derive(InitSpace)]
pub struct DepositInfo {
    pub note: Pubkey,

    pub pool_token: Pubkey,

    pub amount: f64,
}
