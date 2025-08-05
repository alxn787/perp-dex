use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
pub mod states;
pub use states::*;

declare_id!("B2NX28c1xjFMokwTSDDFpoB4XJPXaTJQ53EkeR6xzFsA");

#[program]
pub mod perp_dex {
    use super::*;

    pub fn initialize_market(ctx: Context<InitializeMarket>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}


// #[account]
// pub struct User {
//     pub pubkey: Pubkey,
//     pub orders: [Orders; 8],
// }

// #[account]
// pub struct Market {
//     pub market_index: u64,
//     pub authority: Pubkey,
//     pub base_asset_reserve: u64,
//     pub quote_asset_reserve: u64,
//     pub liquidator_fee: u64,
//     pub max_leverage: u64,
//     pub margin_ratio_initial: u64,
//     pub margin_ratio_maintainance: u64,
//     pub amm: Amm,
// }

// #[account]
// pub struct Amm {
//     pub base_asset_reserve: u64,
//     pub quote_asset_reserve: u64,
// }