use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};

#[account]
pub struct Amm {
    pub oracle : Pubkey,
    pub base_asset_reserve: u64,
    pub quote_asset_reserve: u64,
    pub sqrt_k: u64,
    pub last_funding_rate: u64,
    pub last_funding_rate_ts: UnixTimestamp,
    pub amm_price: u64,
}