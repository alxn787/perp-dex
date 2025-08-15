use anchor_lang::prelude::*;
use crate::utils::constant::*;
use crate::utils::constraint::*;


#[account]
pub struct Order {
    pub market_index: u16,
    pub order_index: u64,
    pub base_asset_amount: u64,
    pub base_asset_amount_filled: u64,
    pub quote_asset_amount_filled: u64,
    pub price: Option<u64>,
    pub direction: PositionDirection,
    pub order_type: OrderType,
    pub leverage: u64,
    pub status: OrderStatus,
    pub order_id: u64,
}

impl Order {
    pub const SIZE: usize = 8 + // discriminator
                           2 + // market_index (u16)
                           8 + // order_index  
                           8 + // base_asset_amount
                           8 + // base_asset_amount_filled
                           8 + // quote_asset_amount_filled
                           8 + // price
                           1 + // direction
                           1 + // order_type
                           8 + // leverage
                           1 + // status
                           8;  // order_id
                           // Total: 69 bytes

    pub fn is_available(&self) -> bool {
        self.status != OrderStatus::Open
    }

    pub fn validate(&self) -> Result<()> {
        require!(self.base_asset_amount >= MIN_ORDER_AMOUNT, crate::utils::error::Perperror::InvalidAmount);
        require!(self.leverage >= MIN_LEVERAGE && self.leverage <= MAX_LEVERAGE, crate::utils::error::Perperror::InvalidLeverage);
        require!(self.price.unwrap() > 0, crate::utils::error::Perperror::InvalidPrice);
        Ok(())
    }

    pub fn opposite(&self) -> PositionDirection {
        match self.direction {
            PositionDirection::Long => PositionDirection::Short,
            PositionDirection::Short => PositionDirection::Long
        }
    }
}