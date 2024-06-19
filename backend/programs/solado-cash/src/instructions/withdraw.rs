use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer, Mint, Token, TokenAccount },
};
use solana_program::hash::hash;
use crate::{
    contants::{ NOTE_SEED, POOL_VAULT_AMOUNT_SEED, POOL_VAULT_SEED },
    errors::AppError,
    state::{ MerkleTree, Note },
    utils::merkle_verify,
};
use anchor_spl::token::Transfer;

#[derive(Accounts)]
pub struct Withdraw<'info> {
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

pub fn withdraw(ctx: Context<Withdraw>, proof: String, root: String, leaf: String) -> Result<()> {
    let merkle_tree = &mut ctx.accounts.merkle_tree_account;

    let hashes = merkle_tree.decode_proof(&proof);
    let root_vec = bs58::decode(root).into_vec().expect("Invalid Base58 string");
    let leaf_vec = bs58::decode(leaf).into_vec().expect("Invalid Base58 string");
    // Convert the u8 vectors to [u8; 32] arrays
    let mut root_array = [0u8; 32];
    let mut leaf_array = [0u8; 32];

    root_array.copy_from_slice(&root_vec);
    leaf_array.copy_from_slice(&leaf_vec);

    if !merkle_verify(hashes, root_array, leaf_array) {
        return Err(AppError::InvalidProof.into());
    }

    msg!("valid");

    Ok(())
}
