use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer, Mint, Token, TokenAccount },
};
use crate::{
    contants::PROOF_SEED,
    errors::AppError,
    generate_proof,
    state::{ MerkleMountainRange, ProofAccount },
};
use anchor_lang::solana_program::hash;
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
    )]
    pub mmr_account: Account<'info, MerkleMountainRange>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + ProofAccount::INIT_SPACE,
        seeds = [
            PROOF_SEED,
            user.to_account_info().key.as_ref(),
            mmr_account.to_account_info().key.as_ref(),
            // this for unique account for each time user deposit
            mmr_account.nodes.len().to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub proof_account: Account<'info, ProofAccount>,

    #[account(mut, token::mint = pool_token, token::authority = mmr_account)]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    if amount <= 0 {
        return Err(AppError::InvalidAmount.into());
    }

    // deposit to pool
    let _ = transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        }),
        amount
    );

    // update merkle tree
    let mmr_account = &mut ctx.accounts.mmr_account;

    let data_to_hash: Vec<u8> = [
        ctx.accounts.user_token_account.to_account_info().key.as_ref(),
        ctx.accounts.pool_token_account.to_account_info().key.as_ref(),
        &amount.to_le_bytes(),
        &mmr_account.nodes.len().to_le_bytes(),
    ].concat();

    // 1. Hash the Input Data
    let leaf_hash: [u8; 32] = hash::hash(data_to_hash.as_slice()).to_bytes();
    mmr_account.nodes.push(leaf_hash);

    // 2. Update Peaks
    let mut leaf_index = mmr_account.nodes.len() - 1;
    let current_leaf_index = leaf_index as u32;

    let mut combined = [0u8; 64];
    while leaf_index > 0 {
        let parent_index: usize = leaf_index >> 1;
        let sibling_index = if (leaf_index & 1) == 0 { leaf_index + 1 } else { leaf_index - 1 };

        if sibling_index < mmr_account.nodes.len() {
            // Combine with Sibling
            combined[..32].copy_from_slice(&mmr_account.nodes[leaf_index]);
            combined[32..].copy_from_slice(&mmr_account.nodes[sibling_index]);
            let parent_hash = hash::hash(&combined).to_bytes();
            mmr_account.nodes.push(parent_hash);

            // Remove Sibling and Parent (if in peaks)
            mmr_account.peaks.retain(
                |&peak| peak != (sibling_index as u32) && peak != (parent_index as u32)
            );

            leaf_index = parent_index; // Move up the tree
        } else {
            // No Sibling - Becomes a New Peak
            mmr_account.peaks.push(leaf_index as u32);
            break;
        }
    }
    ctx.accounts.mmr_account.deposit_count += 1;
    msg!("Deposit Success");

    msg!("Leaf Index: {:?}", current_leaf_index);
    // store proof to proof account
    ctx.accounts.proof_account.owner = *ctx.accounts.user.to_account_info().key;
    ctx.accounts.proof_account.proof = generate_proof(&ctx, current_leaf_index)?;
    ctx.accounts.proof_account.deposit_number = ctx.accounts.mmr_account.deposit_count;

    Ok(())
}
