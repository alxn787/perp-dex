use anchor_lang::prelude::*;

#[account]
pub struct Amm {
    pub oracle : Pubkey,
    pub base_asset_reserve: u64,
    pub quote_asset_reserve: u64,
    sqrt_k: u64,
    pub last_funding_rate: u64,
    
}