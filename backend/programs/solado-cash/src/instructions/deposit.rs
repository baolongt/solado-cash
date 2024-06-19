use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer, Mint, Token, TokenAccount },
};
use solana_program::keccak;
use crate::{
    contants::{ NOTE_SEED, POOL_VAULT_AMOUNT_SEED, POOL_VAULT_SEED },
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
        associated_token::mint = pool_token,
        associated_token::authority = merkle_tree_account
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [POOL_VAULT_SEED, POOL_VAULT_AMOUNT_SEED, pool_token.key().as_ref()],
        bump
    )]
    pub merkle_tree_account: Account<'info, MerkleTree>,

    #[account(
        init,
        payer = user,
        space = 8 + Note::INIT_SPACE,
        seeds = [
            NOTE_SEED,
            user.to_account_info().key.as_ref(),
            merkle_tree_account.to_account_info().key.as_ref(),
            merkle_tree_account.current_leaf_index.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub note_account: Account<'info, Note>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Early validation
    if amount == 0 {
        return Err(AppError::InvalidAmount.into());
    }

    let merkle_tree = &mut ctx.accounts.merkle_tree_account;

    // Pool deposit
    transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        }),
        amount
    )?;

    // Merkle tree update
    let user = ctx.accounts.user.to_account_info();
    let leaf_index = merkle_tree.current_leaf_index;
    let leaf = keccak::hashv(
        &[&user.key().to_bytes(), &leaf_index.to_le_bytes(), &amount.to_le_bytes()]
    );
    merkle_tree.insert(leaf.to_bytes())?;

    // Note creation
    let note = &mut ctx.accounts.note_account;
    note.proof = merkle_tree.generate_proof(leaf_index);
    note.root = merkle_tree.root;
    note.owner = *user.key;
    note.leaf = leaf.to_bytes();

    Ok(())
}
