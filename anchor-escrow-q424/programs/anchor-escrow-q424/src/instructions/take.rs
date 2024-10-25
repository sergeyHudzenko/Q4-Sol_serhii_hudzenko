use anchor_lang::prelude::*; 

// create context
#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(init_if_needed, payer = taker, associated_token_a: mint = mint_a, associated_token_b: mint = mint_b)]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_toke:mint = mint_b, assocated_token:authority = taker)]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
      init_if_needed,
      payer = maker,
      assocated_token:mint = mint_b,
      associated_token:authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, close = maker, has_one = maker, has_one = mint_a, has_one = mint_b,seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref(), bump = escrow.bump])]
    pub escrow: Account<'info, Escrow>,
    #[account(
      mut,
      assocated_token:mint = mint_a,
      associated_token:authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub assocated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}


// Deposit tokens from taker to maker
// Transfer tokens from vault to maker
// close vault account
impl<'info> Take<'info> {
    pub fn deposit(&mut, self) -> Result<()> {
      let cpi_program  = self.token_program.to_account_info();

      let cpi_account = TransferChecked {
        from: self.taker_ata_a.to_account_info(),
        to: self.maker_ata_b.to_account_info(),
      }
    }
}