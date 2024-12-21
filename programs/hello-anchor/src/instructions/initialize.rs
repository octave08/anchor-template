use anchor_lang::prelude::*;

use crate::{constants::ANCHOR_DISCRIMINATOR, state::Memo};

/*
    the Accounts struct is typically named the same thing as the instruction handler, just in TitleCase.
    Eg, the struct with the accounts for add_movie_review() is called AddMovieReview!
*/
#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = signer, space = ANCHOR_DISCRIMINATOR + Memo::INIT_SPACE)]
  pub new_account: Account<'info, Memo>,
  #[account(mut)]
  pub signer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
  msg!("Greetings from: {:?}", ctx.program_id);

  ctx.accounts.new_account.data = data;
  msg!("Changed data to: {}!", data);
  msg!(
    "Account created with data: {}!",
    ctx.accounts.new_account.data
  );

  Ok(())
}
