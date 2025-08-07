use anchor_lang::prelude::*;

#[account]
pub struct State {
    pub admin: Pubkey,
    pub no_of_markets: u64,
    pub perp_fee: u64,
    pub bump: u8,
    pub signer: Pubkey,
    pub signer_bump: u8,
}

impl State {
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 1 + 32 + 1; // 90 bytes total
}