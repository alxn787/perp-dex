use std::ops::Mul;

use anchor_lang::prelude::*;

use crate::utils::Perperror;

#[account]
pub struct Amm {
    pub oracle : Pubkey,
    pub base_asset_reserve: u64,
    pub quote_asset_reserve: u64,
    pub last_funding_rate: u64,
    pub last_funding_rate_ts: i64,
    pub amm_price: u64,
    pub k: u64,
    pub oracle_price_weight: u64, // Weight for oracle price (0-10000, where 10000 = 100%)
    pub last_oracle_update: i64,
}

impl Amm {
    pub const SIZE: usize = 8 + // discriminator
                           32 + // oracle (Pubkey)
                           8 + // base_asset_reserve (u64)
                           8 + // quote_asset_reserve (u64)
                           8 + // last_funding_rate (u64)
                           8 + // last_funding_rate_ts (i64)
                           8 + // amm_price (u64)
                           8 + // k (u64)
                           8 + // oracle_price_weight (u64)
                           8;  // last_oracle_update (i64)
                           // Total: 96 bytes

    pub fn get_bid_price(&self) -> u64 {
        self.quote_asset_reserve / self.base_asset_reserve
    }
    
    pub fn get_ask_price(&self) -> u64 {
        self.base_asset_reserve / self.quote_asset_reserve
    }

    pub fn get_oracle_weighted_price(&self, oracle_price: u64) -> u64 {
        if self.oracle_price_weight == 0 {
            return self.amm_price;
        }
        
        if self.oracle_price_weight >= 10000 {
            return oracle_price;
        }
        
        let amm_weight = 10000 - self.oracle_price_weight;
        
        let weighted_oracle = (oracle_price * self.oracle_price_weight) / 10000;
        let weighted_amm = (self.amm_price * amm_weight) / 10000;
        
        weighted_oracle + weighted_amm
    }

    pub fn update_oracle_price_weight(&mut self, new_weight: u64) -> Result<()> {
        require!(new_weight <= 10000, Perperror::InvalidAmount);
        self.oracle_price_weight = new_weight;
        self.last_oracle_update = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    pub fn calculate_quote_for_base_no_limit(&self, base_amount: u64) -> Result<u64> {
        let new_base_reserve = self.base_asset_reserve
        .checked_sub(base_amount)
        .ok_or(Perperror::ArithmeticOverflow)?;

        let new_quote_reserve = self.k.checked_div(new_base_reserve)
        .ok_or(Perperror::ArithmeticOverflow)?;

        let quote_amount = new_quote_reserve.checked_sub(self.quote_asset_reserve)
        .ok_or(Perperror::ArithmeticOverflow)?;
        Ok(quote_amount)
    }

    pub fn calculate_quote_for_base_with_limit(&self, base_amount: u64, limit_price: u64) -> Result<u64> {
        if limit_price < self.amm_price {
            return Ok(0);
        }
        let new_base_reserve = self.base_asset_reserve
        .checked_sub(base_amount)
        .ok_or(Perperror::ArithmeticOverflow)?;

        let new_quote_reserve = self.k.checked_div(new_base_reserve)
        .ok_or(Perperror::ArithmeticOverflow)?;

        let quote_amount = new_quote_reserve.checked_sub(self.quote_asset_reserve)
        .ok_or(Perperror::ArithmeticOverflow)?;
        Ok(quote_amount)
    }

    pub fn calculate_quote_for_base_with_oracle(&self, base_amount: u64, oracle_price: u64) -> Result<u64> {
        let effective_price = self.get_oracle_weighted_price(oracle_price);
        
        // Use oracle-weighted price for calculation
        let quote_amount = base_amount.checked_mul(effective_price)
            .ok_or(Perperror::ArithmeticOverflow)?;
        
        Ok(quote_amount)
    }
    
    pub fn execute_trade(&mut self, base_amount: u64, quote_amount: u64) -> Result<()> {

        self.base_asset_reserve = self.base_asset_reserve
        .checked_sub(base_amount)
        .ok_or(Perperror::ArithmeticOverflow)?;

        self.quote_asset_reserve = self.quote_asset_reserve
        .checked_add(quote_amount)
        .ok_or(Perperror::ArithmeticOverflow)?;
        
        // Verify k is maintained
        let new_k = self.base_asset_reserve
        .checked_mul(self.quote_asset_reserve)
        .ok_or(Perperror::ArithmeticOverflow)?;
        
        if new_k != self.base_asset_reserve * self.quote_asset_reserve {
            return Err(Perperror::ArithmeticOverflow.into());
        }
        
        Ok(())
    }

    pub fn execute_trade_with_oracle(&mut self, base_amount: u64, oracle_price: u64) -> Result<(u64, u64)> {
        let quote_amount = self.calculate_quote_for_base_with_oracle(base_amount, oracle_price)?;
        
        self.execute_trade(base_amount, quote_amount)?;
        
        Ok((base_amount, quote_amount))
    }
}