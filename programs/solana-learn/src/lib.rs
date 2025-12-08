use anchor_lang::prelude::*;

declare_id!("64odpoZa5Y3WPV5d6xUVgziCQirY5vYYRzSHepmnCRH1");

#[program]
pub mod solana_learn {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
