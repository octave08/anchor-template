use anchor_lang::prelude::*;

declare_id!("2gVBXBHTYKC9HCbCPk7uLjDrY6MvHW5K5NttBS6gSAwn");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        msg!("Account created with data: {}!", ctx.accounts.new_account.data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NewAccount {
    data: u64,
}
