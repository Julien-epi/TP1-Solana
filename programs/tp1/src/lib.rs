use anchor_lang::prelude::*;

declare_id!("7T1gPMxQoJYWsH9uDyRFBTrqfB6C1wtUE9nqpeSzjawr");

#[program]
pub mod tp1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
