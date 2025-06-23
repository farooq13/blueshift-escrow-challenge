  use anchor_lang::prelude::*;
  use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, close_account, TransferChecked, CloseAccount},
  };

  use crate::state::Escrow;
  use crate::error::EscrowError;


  #[derive(Accounts)]
  #[instruction()]
  pub struct Refund<'info> {
    /// The user that created the escrow (maker)
    #[account(mut)]
    pub maker: Signer<'info>,

    /// The escrow holding the exhchange terms
    #[account(
      mut,
      close = maker,
      seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
      bump = escrow.bump,
      has_one = maker @ EscrowError::InvalidMaker,
      has_one = ming_a @ EscrowError::InvalidMintA
    )]
    pub escrow: Box<Account<'info>, Escrow>,

    /// Mint of token A (what the maker deposited)
    pub mint_a: Box<InterfaceAccount<'info>, Mint>,

    /// Token account owned by the escrow where token A was stroed
    #[account(
      mut,
      associated_token::mint = mint_a,
      associated_token::authority = escrow,
      associated_token::token_program = token_program
    )]
    pub vault: Box<InterfaceAccount<'info>, TokenAccount>,

    /// Maker's ATA to receive token A back
    #[account(
      init_if_needed,
      payer = maker,
      associated_token::mint = mint_a,
      associated_token::authority = maker,
      associated_token::token_program = token_program
    )]
    pub maker_ata_a: Box<InterfaceAccount<'info>, TokenAccount>,

    /// Programs
    associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
  }


  impl<'info> Refund<'info> {
    pub fn handler(ctx: Context<Refund>) -> Result<()> {
      let accounts = ctx.accounts;

      // Seed signer
      let signer_seeds: &[&[u8]] = &[
        b"escrow",
        accounts.maker.key.as_ref(),
        &accounts.escrow.seed.to_le_bytes(),
        &[accouts.escrow.bump],
      ];

      // 1. Transfer token A from vault -> maker_ata_a
      transfer_checked(
        CpiContext::new_with_signer(
          accounts.token_program.to_account_info(),
          TransferChecked {
            from: accounts.vault.to_account_info(),
            to: accounts.maker_ata_a.to_account_info(),
            mint: accounts.mint_a.to_account_info(),
            authority: accounts.to_acount_info()
          },
          &[singer_seeds],
        ),
        accounts.vault.amount,
        accounts.mint_a.decimals
      )?;

      // 2. Close the vault, refunding rent to the maker
      close_account(
        CpiContext::new_with_signer(
          accounts.token_program.to_account_info(),
          CloseAccount {
            account: accounts.vault.to_account_info(),
            authority: accounts.escrow.to_account_info(),
            destination: accounts.maker.to_account_info()
          },
          &[signer_seeds],
        ),
      )?;

      Ok(())
    }
  }