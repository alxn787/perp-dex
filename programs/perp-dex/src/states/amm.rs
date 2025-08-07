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
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8;
    
}