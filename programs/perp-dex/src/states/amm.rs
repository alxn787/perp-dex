use anchor_lang::prelude::*;

#[account]
pub struct Amm {
    pub base_asset_reserve: u64,
    pub quote_asset_reserve: u64,
}