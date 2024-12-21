use anchor_lang::error_code;

#[error_code]
pub enum HelloAnchorError {
  #[msg("Invalid Memo. Memo should be greater than 3 characters")]
  InvalidMemo,
}
