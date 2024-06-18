use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer, Mint, Token, TokenAccount },
};
use solana_program::hash::hash;
use crate::{
    contants::{ POOL_VAULT_AMOUNT_SEED, POOL_VAULT_SEED, NOTE_SEED },
    errors::AppError,
    state::{ MerkleTree, Note },
};
use anchor_spl::token::Transfer;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub pool_token: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = pool_token,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [POOL_VAULT_SEED, POOL_VAULT_AMOUNT_SEED, pool_token.key().as_ref()],
        bump
    )]
    pub merkle_tree_account: Account<'info, MerkleTree>,

    #[account(
        mut,
        associated_token::mint = pool_token,
        associated_token::authority = merkle_tree_account
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
