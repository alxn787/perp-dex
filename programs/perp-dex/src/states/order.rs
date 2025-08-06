use anchor_lang::prelude::*;

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Eq, Default)]
pub enum PositionDirection {
    #[default]
    Long,
    Short,
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Eq, Default)]
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

impl Order {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 8 + 8 + 1 + 1 + 8; // 58 bytes
}