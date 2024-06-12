use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::{ Mint, Token, TokenAccount } };

use crate::{ contants::{ POOL_VAULT_AMOUNT_SEED, POOL_VAULT_SEED }, state::DepositInfo };

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub pool_token: Account<'info, Mint>,

    #[account(token::mint = pool_token, token::authority = pool_token_account)]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn deposit(_ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}
