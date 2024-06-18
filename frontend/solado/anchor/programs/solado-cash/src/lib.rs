use anchor_lang::prelude::*;

declare_id!("esfrnUs49Ng4j1ZVBzjhsKYMrhrufr8sh4PYWXFH4rm");

#[program]
pub mod solado_cash {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
