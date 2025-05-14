use anchor_lang::prelude::*;

declare_id!("4YvPaNJ5DkHfet9vBQopFbVs9g1jUTg6o58PL2bX58Z1");

#[program]
pub mod contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
