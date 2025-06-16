use anchor_lang::prelude::*;

declare_id!("8fiKg6C7WiBKLdy8jMCt9BDBsjpSerRXaksfg2mUcnLR");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
