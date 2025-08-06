use anchor_lang::prelude::*;
use crate::order::Order;

#[account]
pub struct User {
    pub authority: Pubkey,
    pub orders: [Order; 8],
    pub total_collateral: u64,
}