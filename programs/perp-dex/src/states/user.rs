use anchor_lang::prelude::*;
use crate::order::Order;

#[account]
pub struct User {
    pub pubkey: Pubkey,
    pub orders: [Order; 8],
}