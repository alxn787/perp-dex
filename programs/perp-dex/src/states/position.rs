use anchor_lang::prelude::*;
use crate::utils::error::Perperror;

#[account]
pub struct PerpPosition {
    pub last_cumulative_funding_rate: i64,
    pub market_index: u64,
    pub base_asset_amount: u64,
    pub quote_asset_amount: u64,
    pub open_orders: u8,
    pub pnl: i64,
}

impl PerpPosition {
    pub const SIZE: usize = 8 + // discriminator
                           8 + // last_cumulative_funding_rate (i64)
                           8 + // market_index (u64)
                           8 + // base_asset_amount (u64)
                           8 + // quote_asset_amount (u64)
                           1 + // open_orders (u8)
                           8;  // pnl (i64)
                           // Total: 57 bytes

    pub fn default() -> Self {
        Self {
            last_cumulative_funding_rate: 0,
            market_index: 0,
            base_asset_amount: 0,
            quote_asset_amount: 0,
            open_orders: 0,
            pnl: 0,
        }
    }

    pub fn is_available(&self) -> bool {
        self.open_orders == 0
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