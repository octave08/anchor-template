use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct Memo {
  #[max_len(256)]
  pub data: String,
}
