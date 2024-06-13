use anchor_lang::prelude::*;
use instructions::*;

mod contants;
mod errors;
mod instructions;
mod state;

declare_id!("GtviumFyW3zVqLCmxJZpBSbQT4LePTyMAubHVZA4DT7X");

#[program]
pub mod solado_cash {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }
}
