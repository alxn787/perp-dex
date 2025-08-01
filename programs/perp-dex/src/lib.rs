use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

declare_id!("B2NX28c1xjFMokwTSDDFpoB4XJPXaTJQ53EkeR6xzFsA");

#[program]
pub mod perp_dex {
    use super::*;

    pub fn initialize_market(ctx: Context<InitializeMarket>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

