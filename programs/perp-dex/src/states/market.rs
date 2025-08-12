use anchor_lang::prelude::*;
use crate::amm::Amm;

#[account]
pub struct PerpMarket {
    pub market_index: u64,
    pub authority: Pubkey,
    pub liquidator_fee: u64,
    pub max_leverage: u64,
    pub margin_ratio_initial: u64,
    pub margin_ratio_maintainance: u64,
    pub amm: Amm,
    pub bump : u8,
}

impl PerpMarket {
    pub const SIZE: usize = 8 + // discriminator
                           8 + // market_index (u64)
                           32 + // authority (Pubkey)
                           8 + // liquidator_fee (u64)
                           8 + // max_leverage (u64)
                           8 + // margin_ratio_initial (u64)
                           8 + // margin_ratio_maintainance (u64)
                           Amm::SIZE + // amm (Amm)
                           1;  // bump (u8)
                           // Total: 81 + Amm::SIZE bytes
}