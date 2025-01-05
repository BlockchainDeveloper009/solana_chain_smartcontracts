use anchor_lang::prelude::*;

declare_id!("EnZd9fh47gDDzCyCs2r8BQWbgm1cBQRc6briPFQG1SM7");

#[program]
pub mod hello_smartcontracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("hello_smartcontracts Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
