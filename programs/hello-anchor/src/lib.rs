use anchor_lang::prelude::*;

mod constants;
pub use constants::*;

mod state;
pub use state::*;

mod instructions;
use instructions::*;

mod errors;
pub use errors::*;

declare_id!("2gVBXBHTYKC9HCbCPk7uLjDrY6MvHW5K5NttBS6gSAwn");

#[program]
pub mod hello_anchor {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>, data: String) -> Result<()> {
    require!(data.len() > 3, HelloAnchorError::InvalidMemo);

    instructions::initialize(ctx, data)
  }
}
