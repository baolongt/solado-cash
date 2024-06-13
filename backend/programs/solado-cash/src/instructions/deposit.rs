use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::{ self, Mint, Token, TokenAccount } };

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

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Transfer tokens from the user's account to the pool token account
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.user.to_account_info().clone(),
        to: ctx.accounts.pool_token_account.to_account_info().clone(),
        authority: ctx.accounts.user.to_account_info().clone(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info().clone();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}
