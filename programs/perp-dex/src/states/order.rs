use anchor_lang::prelude::*;
use crate::borsh::BorshSerialize;
use crate::borsh::BorshDeserialize;

#[derive(Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Debug, Eq, Default)]
pub enum PositionDirection {
    #[default]
    Long,
    Short,
}

#[derive(Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Debug, Eq, Default)]
pub enum OrderType {
    Market,
    #[default]
    Limit,
}

#[account]
pub struct Order {
    pub market_index: u64,
    pub order_index: u64,
    pub base_asset_amount: u64,
    pub base_asset_amount_filled: u64,
    pub quote_asset_amount_filled: u64,
    pub price: u64,
    pub direction: PositionDirection,
    pub order_type: OrderType,
    pub leverage: u64,
}