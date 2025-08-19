use anchor_lang::prelude::*;
use crate::utils::error::Perperror;

#[account]
pub struct Oracle {
    pub market_index: u16,
    pub authority: Pubkey,
    pub price: u64,
    pub last_update_ts: i64,
    pub confidence_interval: u64,
    pub max_price_deviation: u64,
    pub bump: u8,
}

impl Oracle {
    pub const SIZE: usize = 8 + // discriminator
                           2 + // market_index (u16)
                           32 + // authority (Pubkey)
                           8 + // price (u64)
                           8 + // last_update_ts (i64)
                           8 + // confidence_interval (u64)
                           8 + // max_price_deviation (u64)
                           1;  // bump (u8)
                           // Total: 75 bytes

    pub fn new(
        market_index: u16,
        authority: Pubkey,
        initial_price: u64,
        confidence_interval: u64,
        max_price_deviation: u64,
        bump: u8,
    ) -> Self {
        Self {
            market_index,
            authority,
            price: initial_price,
            last_update_ts: Clock::get().unwrap().unix_timestamp,
            confidence_interval,
            max_price_deviation,
            bump,
        }
    }

    pub fn update_price(&mut self, new_price: u64, authority: &Pubkey) -> Result<()> {
        require!(self.authority == *authority, Perperror::Unauthorized);
        
        // Check if price deviation is within acceptable range
        let price_deviation = self.calculate_price_deviation(new_price);
        require!(
            price_deviation <= self.max_price_deviation,
            Perperror::PriceDeviationTooHigh
        );

        self.price = new_price;
        self.last_update_ts = Clock::get().unwrap().unix_timestamp;
        
        Ok(())
    }

    pub fn calculate_price_deviation(&self, new_price: u64) -> u64 {
        if self.price == 0 {
            return 0;
        }
        
        let deviation = if new_price > self.price {
            new_price - self.price
        } else {
            self.price - new_price
        };
        
        // Calculate percentage deviation (in basis points)
        (deviation * 10000) / self.price
    }

    pub fn is_price_stale(&self) -> bool {
        let current_ts = Clock::get().unwrap().unix_timestamp;
        current_ts - self.last_update_ts > self.confidence_interval as i64
    }

    pub fn get_price(&self) -> Result<u64> {
        require!(!self.is_price_stale(), Perperror::StaleOraclePrice);
        Ok(self.price)
    }

    pub fn get_twap_price(&self, duration: i64) -> Result<u64> {
        // Simple TWAP implementation - in production you'd want more sophisticated logic
        let current_ts = Clock::get().unwrap().unix_timestamp;
        let time_diff = current_ts - self.last_update_ts;
        
        if time_diff > duration {
            return Err(Perperror::StaleOraclePrice.into());
        }
        
        Ok(self.price)
    }
} 