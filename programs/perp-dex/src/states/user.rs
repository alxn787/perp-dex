use anchor_lang::prelude::*;
use super::Order;
use super::PerpPosition;
#[account]
pub struct User {
    pub authority: Pubkey,
    pub orders: [Order; 16],
    pub total_collateral: u64,
    pub perp_positions: [PerpPosition; 8],
    pub next_order_id: u64,
    pub open_orders: u64,
    pub account_id: u16,
}

impl User {
    pub const SIZE: usize = 8 + // discriminator
                           32 + // authority (Pubkey)
                           16 * Order::SIZE + // orders [Order; 16]
                           8 + // total_collateral (u64)
                           8 * PerpPosition::SIZE + // perp_positions [PerpPosition; 8]
                           8 + // next_order_id (u64)
                           8 + // open_orders (u64)
                           2;  // account_id (u16)
                           // Total: 58 + 16*Order::SIZE + 8*PerpPosition::SIZE bytes
}
