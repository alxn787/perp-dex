use anchor_lang::prelude::*;

#[account]
pub struct Market {
    pub bump: u8,
    pub authority: Pubkey,
    pub market_index: u64,
}