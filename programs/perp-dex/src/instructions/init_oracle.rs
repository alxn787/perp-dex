use anchor_lang::prelude::*;
use crate::{Oracle, State};

#[derive(Accounts)]
#[instruction(market_index: u16, initial_price: u64, confidence_interval: u64, max_price_deviation: u64)]
pub struct InitializeOracle<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    
    #[account(
        init,
        payer = authority,
        space = Oracle::SIZE,
        seeds = [b"oracle", market_index.to_le_bytes().as_ref()],
        bump
    )]
    pub oracle: Account<'info, Oracle>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(market_index: u16)]
pub struct UpdateOraclePrice<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    
    #[account(
        mut,
        seeds = [b"oracle", market_index.to_le_bytes().as_ref()],
        bump = oracle.bump,
        constraint = oracle.authority == authority.key()
    )]
    pub oracle: Account<'info, Oracle>,
    
    pub authority: Signer<'info>,
}

pub fn handle_initialize_oracle(
    ctx: Context<InitializeOracle>,
    market_index: u16,
    initial_price: u64,
    confidence_interval: u64,
    max_price_deviation: u64,
) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;
    
    oracle.market_index = market_index;
    oracle.authority = ctx.accounts.authority.key();
    oracle.price = initial_price;
    oracle.last_update_ts = Clock::get().unwrap().unix_timestamp;
    oracle.confidence_interval = confidence_interval;
    oracle.max_price_deviation = max_price_deviation;
    oracle.bump = ctx.bumps.oracle;
    
    msg!("Oracle initialized for market {} with price {}", market_index, initial_price);
    Ok(())
}

pub fn handle_update_oracle_price(
    ctx: Context<UpdateOraclePrice>,
    _market_index: u16,
    new_price: u64,
) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;
    
    oracle.update_price(new_price, &ctx.accounts.authority.key())?;
    
    msg!("Oracle price updated to {} for market {}", new_price, oracle.market_index);
    Ok(())
} 