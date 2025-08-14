use anchor_lang::prelude::*;
use crate::utils::error::Perperror;
use crate::utils::constraint::PositionDirection;

#[account]
pub struct PerpPosition {
    pub last_cumulative_funding_rate: i64,
    pub market_index: u64,
    pub base_asset_amount: i64,
    pub quote_asset_amount: i64,
    pub open_orders: u8,
    pub pnl: i64,
    pub bids: u64,
    pub asks: u64,
}

impl PerpPosition {
    pub const SIZE: usize = 8 + // discriminator
                           8 + // last_cumulative_funding_rate (i64)
                           8 + // market_index (u64)
                           8 + // base_asset_amount (i64)
                           8 + // quote_asset_amount (i64)
                           1 + // open_orders (u8)
                           8 + // pnl (i64)
                           8 + // bids (u64)
                           8;  // asks (u64)
                           // Total: 65 bytes

    pub fn default() -> Self {
        Self {
            last_cumulative_funding_rate: 0,
            market_index: 0,
            base_asset_amount: 0,
            quote_asset_amount: 0,
            open_orders: 0,
            pnl: 0,
            bids: 0,
            asks: 0,    
        }
    }

    pub fn is_available(&self) -> bool {
        self.open_orders == 0
    }

    pub fn can_add_order(&self) -> bool {
        self.open_orders < u8::MAX
    }

    pub fn add_order(&mut self) -> Result<()> {
        require!(self.can_add_order(), Perperror::MaxNumberOfOrders);
        self.open_orders += 1;
        Ok(())
    }

    pub fn remove_order(&mut self) -> Result<()> {
        require!(self.open_orders > 0, Perperror::UserHasNoOrderInMarket);
        self.open_orders -= 1;
        Ok(())
    }
}

pub(crate) type PerpPositions = [PerpPosition; 8];

pub fn add_new_position(
    user_positions: &mut PerpPositions,
    market_index: u16,
) -> Result<usize> {
    let new_position_index = user_positions
        .iter()
        .position(|market_position| market_position.is_available())
        .ok_or(Perperror::MaxNumberOfPositions)?;

    let new_market_position = PerpPosition {
        market_index: market_index as u64,
        ..PerpPosition::default()
    };

    user_positions[new_position_index] = new_market_position;

    Ok(new_position_index)
}

pub fn get_position_index(user_positions: &PerpPositions, market_index: u16) -> Result<usize> {
    let position_index = user_positions
        .iter()
        .position(|market_position| market_position.market_index == market_index as u64);

    match position_index {
        Some(position_index) => Ok(position_index),
        None => Err(Perperror::UserHasNoPositionInMarket.into()),
    }
}

pub fn update_bids_and_asks(user_positions: &mut PerpPosition, direction: PositionDirection, base_asset_amount: u64) -> Result<()> {
    match direction {
        PositionDirection::Long => {
            user_positions.bids += base_asset_amount;
        }
        PositionDirection::Short => {
            user_positions.asks += base_asset_amount;
        }
    }
    Ok(())
}