use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub pubkey: Pubkey,
    pub orders: [Orders; 8],
}