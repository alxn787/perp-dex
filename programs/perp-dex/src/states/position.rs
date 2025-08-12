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

impl PerpPosition {
    pub const SIZE: usize = 8 + // discriminator
                           8 + // last_cumulative_funding_rate (i64)
                           8 + // market_index (u64)
                           8 + // base_asset_amount (u64)
                           8 + // quote_asset_amount (u64)
                           1 + // open_orders (u8)
                           8;  // pnl (i64)
                           // Total: 57 bytes
}