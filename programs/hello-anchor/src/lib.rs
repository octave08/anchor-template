use anchor_lang::prelude::*;

mod constants;


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

/* 
    the Accounts struct is typically named the same thing as the instruction handler, just in TitleCase. 
    Eg, the struct with the accounts for add_movie_review() is called AddMovieReview!
*/
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = constants::ANCHOR_DISCRIMINATOR + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NewAccount {
    data: u64,
}
