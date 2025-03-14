use anchor_lang::prelude::*;

declare_id!("AgGGFSA5z1KBaNKvasJdeZ3Wao4rjTwS9MexK1D9AHWS");

#[program]
pub mod solsec {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
