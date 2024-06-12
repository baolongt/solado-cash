use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, Mint, TokenAccount };

use crate::contants::{ POOL_VAULT_AMOUNT_SEED, POOL_VAULT_SEED };

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub pool_token: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        seeds = [POOL_VAULT_SEED, POOL_VAULT_AMOUNT_SEED, pool_token.key().as_ref()],
        bump,
        token::mint = pool_token,
        token::authority = pool_vault
    )]
    pub pool_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}
