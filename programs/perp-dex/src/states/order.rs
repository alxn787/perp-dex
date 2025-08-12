use anchor_lang::prelude::*;

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Eq, Default)]
pub enum OrderType {
    Market,
    #[default]
    Limit,
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Eq, Default)]
pub enum OrderStatus {
    #[default]
    Open,
    Filled,
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Eq, Default)]
pub enum PositionDirection {
    #[default]
    Long,
    Short,
}

#[account]
pub struct Order {
    pub market_index: u16,
    pub order_index: u64,
    pub base_asset_amount: u64,
    pub base_asset_amount_filled: u64,
    pub quote_asset_amount_filled: u64,
    pub price: u64,
    pub direction: PositionDirection,
    pub order_type: OrderType,
    pub leverage: u64,
    pub status: OrderStatus,
    pub order_id: u64,
}

impl Order {
    pub const SIZE: usize = 8 + // discriminator
                           8 + // market_index
                           8 + // order_index  
                           8 + // base_asset_amount
                           8 + // base_asset_amount_filled
                           8 + // quote_asset_amount_filled
                           8 + // price
                           1 + // direction
                           1 + // order_type
                           8 + // leverage
                           1;  // status
                           // Total: 67 bytes
}

impl Order {
    pub fn is_available(&self) -> bool {
        self.status != OrderStatus::Open
    }
}