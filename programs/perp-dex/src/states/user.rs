use std::u64;

use anchor_lang::prelude::*;
use super::Order;
use super::PerpPosition;
use crate::utils::constant::*;
use crate::utils::constraint::OrderStatus;

#[account]
pub struct User {
    pub authority: Pubkey,
    pub orders: [Order; MAX_ORDERS_PER_USER],
    pub total_collateral: u64,
    pub perp_positions: [PerpPosition; MAX_POSITIONS_PER_USER],
    pub next_order_id: u64,
    pub open_orders: u64,
    pub account_id: u16,
}

impl User {
    pub const SIZE: usize = 8 + // discriminator
                           32 + // authority
                           (Order::SIZE * MAX_ORDERS_PER_USER) + // orders array
                           8 + // total_collateral
                           (PerpPosition::SIZE * MAX_POSITIONS_PER_USER) + // perp_positions array
                           8 + // next_order_id
                           8; // open_orders
                           // Total: variable based on array sizes

    pub fn can_add_order(&self) -> bool {
        self.open_orders < MAX_ORDERS_PER_USER as u64
    }

    pub fn can_add_position(&self) -> bool {
        self.perp_positions.iter().any(|pos| pos.is_available())
    }

    pub fn get_last_order_id(&self) -> u64{
        if(self.next_order_id == 1){
            u64::MAX
        }else {
            self.next_order_id - 1
        }
    }

    pub fn get_order(&self, order_id: u64) -> Option<&Order>{
        self.orders.iter().find(|order| order.order_id == order_id && order.status == OrderStatus::Open)
    }
}