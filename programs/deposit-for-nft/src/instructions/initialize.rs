use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Init {}


pub fn handle(ctx: Context<Init>) -> Result<()> {

    Ok(())
}