use anchor_lang::prelaude::*;


#[error_code]
pub enum EscrowError {
  #[msg("Invalid amount")]
  InvalidAmount,
  #[msg("Invalid maker")]
  InvalidMaker,
  #[msg("Invalid mint a")]
  InvalidMintA,
  #[msg("Invalid mint b")]
  InvalidMintB,

}

