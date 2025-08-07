use anchor_lang::prelude::*;
use crate::order::Order;
use crate::position::PerpPosition;
#[account]
pub struct User {
    pub authority: Pubkey,
    pub orders: [Order; 32],
    pub total_collateral: u64,
    pub perp_positions: [PerpPosition; 8],
}