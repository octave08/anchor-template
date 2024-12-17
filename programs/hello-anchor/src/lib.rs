use anchor_lang::prelude::*;

declare_id!("2gVBXBHTYKC9HCbCPk7uLjDrY6MvHW5K5NttBS6gSAwn");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
