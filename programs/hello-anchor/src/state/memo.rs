use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Memo {
    pub data: u64,
}