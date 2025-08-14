use anchor_lang::prelude::*;
pub mod instructions;
pub use instructions::*;
pub mod states;
pub use states::*;
pub mod utils;
pub use utils::constraint::{OrderParams, InitializeMarketParams};


declare_id!("B2NX28c1xzFMokwTSDDFpoB4XJPXaTJQ53EkeR6xzFsA");

#[program]
pub mod perp_dex {
    use super::*;

    pub fn initializestate(ctx: Context<InitializeState>, perp_fee: u64) -> Result<()> {
        instructions::initialize_state(ctx, perp_fee)
    }
    pub fn initialize_market(ctx: Context<InitializePerpMarket>, market_params: InitializeMarketParams) -> Result<()> {
        handle_initialize_perp_market(ctx, market_params)
    }
    pub fn place_order(ctx: Context<PlaceOrder>, params: OrderParams) -> Result<()> {
        handle_place_order(ctx, params)
    }

    pub fn fill_order(ctx: Context<FillOrder>, order_id: Option<u64>) -> Result<()> {
        handle_fill_order(ctx, order_id)
    }
}

