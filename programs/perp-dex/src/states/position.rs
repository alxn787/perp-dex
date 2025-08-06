use anchor_lang::prelude::*;

#[account]
pub struct PerpPosition {
    pub last_cumulative_funding_rate: i64,
    pub market_index: u64,
    pub base_asset_amount: u64,
    pub quote_asset_amount: u64,
    pub open_orders: u8,
    pub pnl: i64,
}