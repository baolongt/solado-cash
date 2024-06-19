use anchor_lang::prelude::*;
use instructions::*;

mod contants;
mod errors;
mod instructions;
mod state;
mod utils;

declare_id!("GtviumFyW3zVqLCmxJZpBSbQT4LePTyMAubHVZA4DT7X");

#[program]
pub mod solado_cash {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::_deposit(ctx, amount)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        proof: String,
        root: String,
        leaf: String
    ) -> Result<()> {
        instructions::withdraw(ctx, proof, root, leaf)
    }
}
