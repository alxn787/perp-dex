use anchor_lang::prelude::*;

#[account]
pub struct Amm {
    pub oracle : Pubkey,
    pub base_asset_reserve: u64,
    pub quote_asset_reserve: u64,
    pub last_funding_rate: u64,
    pub last_funding_rate_ts: i64,
    pub amm_price: u64,
}

impl Amm {
    pub const SIZE: usize = 8 + // discriminator
                           32 + // oracle (Pubkey)
                           8 + // base_asset_reserve (u64)
                           8 + // quote_asset_reserve (u64)
                           8 + // last_funding_rate (u64)
                           8 + // last_funding_rate_ts (i64)
                           8;  // amm_price (u64)
                           // Total: 80 bytes
}