use anchor_lang::prelude::*;

declare_id!("B2NX28c1xjFMokwTSDDFpoB4XJPXaTJQ53EkeR6xzFsA");

#[program]
pub mod perp_dex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
