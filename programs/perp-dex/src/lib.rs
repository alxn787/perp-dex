use anchor_lang::prelude::*;
pub mod instructions;
pub use instructions::*;
pub mod states;
pub use states::*;
pub mod utils;
pub use utils::*;

declare_id!("B2NX28c1xjFMokwTSDDFpoB4XJPXaTJQ53EkeR6xzFsA");

#[program]
pub mod perp_dex {
    use super::*;

    pub fn init_state(ctx: Context<InitializeState>, perp_fee: u64) -> Result<()> {
        instructions::initialize_state(ctx, perp_fee)
    }
    pub fn initialize_market(ctx: Context<InitializePerpMarket>, market_params: InitializeMarketParams) -> Result<()> {
        handle_initialize_perp_market(ctx, market_params)
    }
}
