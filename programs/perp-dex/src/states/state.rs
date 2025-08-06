use anchor_lang::prelude::*;

#[account]
pub struct State {
    pub admin: Pubkey,
    pub no_of_markets: u64,
}