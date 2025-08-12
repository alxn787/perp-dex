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
    pub fn init_market(ctx: Context<InitializePerpMarket>, market_index: u64, base_asset_reserve: u64, quote_asset_reserve: u64, liquidator_fee: u64, max_leverage: u64, margin_ratio_initial: u64, margin_ratio_maintainance: u64) -> Result<()> {
        instructions::handle_initialize_perp_market(ctx, market_index, base_asset_reserve, quote_asset_reserve, liquidator_fee, max_leverage, margin_ratio_initial, margin_ratio_maintainance)
    }
}
