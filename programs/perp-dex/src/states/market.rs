use anchor_lang::prelude::*;
use crate::amm::Amm;

#[account]
pub struct Market {
    pub market_index: u64,
    pub authority: Pubkey,
    pub base_asset_reserve: u64,
    pub quote_asset_reserve: u64,
    pub liquidator_fee: u64,
    pub max_leverage: u64,
    pub margin_ratio_initial: u64,
    pub margin_ratio_maintainance: u64,
    pub amm: &Amm,
}