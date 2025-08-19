use anchor_lang::prelude::*;
pub mod instructions;
pub use instructions::*;
pub mod states;
pub use states::*;
pub mod utils;
pub use utils::constraint::*;


declare_id!("B2NX28c1xzFMokwTSDDFpoB4XJPXaTJQ53EkeR6xzFsA");

#[program]
pub mod perp_dex {
    use super::*;

    pub fn initialize_state(ctx: Context<InitializeState>, perp_fee: u64) -> Result<()> {
        instructions::initialize_state(ctx, perp_fee)
    }
    pub fn initialize_market(ctx: Context<InitializePerpMarket>, market_params: InitializeMarketParams) -> Result<()> {
        handle_initialize_perp_market(ctx, market_params)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, account_id:u16) -> Result<()> {
        handle_initialize_user(ctx, account_id)
    } 

    pub fn deposit(ctx: Context<Deposit>, market_index: u16, amount: u64) -> Result<()> {
        handle_deposit(ctx, market_index, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, market_index: u16, amount: u64) -> Result<()> {
        handle_withdraw(ctx, market_index, amount)
    }

    pub fn place_order(ctx: Context<PlaceOrder>, order_params: OrderParams) -> Result<()> {
        handle_place_order(ctx, order_params)
    }

    pub fn fill_order(ctx: Context<FillOrder>, order_id: Option<u64>) -> Result<()> {
        handle_fill_order(ctx, order_id)
    }

    pub fn initialize_oracle(
        ctx: Context<InitializeOracle>,
        market_index: u16,
        initial_price: u64,
        confidence_interval: u64,
        max_price_deviation: u64,
    ) -> Result<()> {
        handle_initialize_oracle(ctx, market_index, initial_price, confidence_interval, max_price_deviation)
    }

    pub fn update_oracle_price(
        ctx: Context<UpdateOraclePrice>,
        market_index: u16,
        new_price: u64,
    ) -> Result<()> {
        handle_update_oracle_price(ctx, market_index, new_price)
    }
}

