use anchor_lang::prelude::*;

declare_id!("8KTv6JKDHuhcWwt37MJuU3qDrpYnZHQ6bDQa4EVcT4oU");

#[program]
pub mod todo {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
