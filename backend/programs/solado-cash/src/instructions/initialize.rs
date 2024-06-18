use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::{ Mint, Token, TokenAccount } };
use crate::{
    contants::{ POOL_VAULT_AMOUNT_SEED, POOL_VAULT_SEED },
    state::MerkleTree,
    utils::zero,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub pool_token: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        space = 8 + MerkleTree::INIT_SPACE,
        seeds = [POOL_VAULT_SEED, POOL_VAULT_AMOUNT_SEED, pool_token.key().as_ref()],
        bump
    )]
    pub merkle_tree_account: Account<'info, MerkleTree>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = pool_token,
        associated_token::authority = merkle_tree_account
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let merkle_tree = &mut ctx.accounts.merkle_tree_account;

    merkle_tree.levels = 8;
    merkle_tree.current_leaf_index = 1;
    // Initialize empty nodes
    for level in 0..merkle_tree.levels {
        let zero_hash_for_level = zero((level + 1).into());
        let num_nodes_at_level = (2u64).pow(level as u32);
        let level_start_index = (2u64).pow(level as u32) - 1; // Calculate correct starting index for the level

        for i in 0..num_nodes_at_level {
            let node_index = level_start_index + i;
            merkle_tree.nodes.insert(node_index as usize, zero_hash_for_level);
        }

        if level == merkle_tree.levels {
            merkle_tree.root = zero_hash_for_level;
        }
    }

    Ok(())
}
