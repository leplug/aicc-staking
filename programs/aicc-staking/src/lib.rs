use anchor_lang::prelude::*;

declare_id!("EAzy8EwEMYbPiA6BnVnqRAtemwQWABGbEJqFtgvqUST7");

#[program]
pub mod aicc_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
