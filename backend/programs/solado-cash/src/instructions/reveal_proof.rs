// use anchor_lang::{ prelude::{ Account, Program, Signer }, system_program::System };
// use anchor_lang::prelude::*;
// use crate::{ errors::AppError, state::{ MerkleMountainRange, ProofAccount } };

// #[derive(Accounts)]
// pub struct RevealProof<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,

//     #[account(mut,
//         constraint = proof_account.owner == user.key() @ AppError::InvalidAuthority
//     )]
//     pub proof_account: Account<'info, ProofAccount>,

//     #[account(mut)]
//     pub mmr_account: Account<'info, MerkleMountainRange>,

//     pub system_program: Program<'info, System>,
// }

// pub fn reveal_proof(ctx: Context<RevealProof>) -> Result<()> {
//     let proof_account = &mut ctx.accounts.proof_account;
//     let mmr_account = &ctx.accounts.mmr_account;

//     Ok(())
// }
