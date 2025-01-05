use anchor_lang::prelude::*;

declare_id!("49WRZcDBqSiLMzVzuQRNGsvTsbfhg59m6deu2ndZ1RG3");

#[program]
pub mod solana_chain_smartcontracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
