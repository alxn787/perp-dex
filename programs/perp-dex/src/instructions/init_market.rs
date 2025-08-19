use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::states::*;
use crate::utils::*;
use crate::utils::constant::*;

#[derive(Accounts)]
pub struct InitializePerpMarket<'info> {
    
    #[account(init,
        payer = admin,
        space = PerpMarket::SIZE,
        seeds = [
            b"perp_market".as_ref(),state.no_of_markets.to_le_bytes().as_ref() 
              
        ],
        bump
    )]
    pub market: Account<'info, PerpMarket>,

    pub perp_market_mint: Account<'info, Mint>,

    #[account(init,
        payer = admin,
        seeds = [
            b"perp_market_vault".as_ref(),state.no_of_markets.to_le_bytes().as_ref()  
        ],
        bump,
        token::mint = perp_market_mint,
        token::authority = drift_signer,
    )]
    pub perp_market_vault: Account<'info, TokenAccount>,

    #[account(mut,
        constraint = drift_signer.key() == state.signer.key()
    )]
    /// CHECK: checked in `initialize`
    pub drift_signer: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub state: Account<'info, State>,

    /// CHECK: checked in `initialize`
    pub oracle: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}


pub fn handle_initialize_perp_market(ctx: Context<InitializePerpMarket>,params:InitializeMarketParams) -> Result<()> {
    require!(params.market_index == ctx.accounts.state.no_of_markets, Perperror::InvalidMarketIndex);
    require!(params.max_leverage <= MAX_LEVERAGE, Perperror::InvalidLeverage);
    require!(params.base_asset_reserve > 0, Perperror::InvalidAmount);
    require!(params.quote_asset_reserve > 0, Perperror::InvalidAmount);
    
    let clock = Clock::get().unwrap();
    let market = &mut ctx.accounts.market;
    market.market_index = params.market_index;
    market.authority = ctx.accounts.admin.key();
    market.liquidator_fee = params.liquidator_fee;
    market.max_leverage = params.max_leverage;
    market.margin_ratio_initial = params.margin_ratio_initial;
    market.margin_ratio_maintainance = params.margin_ratio_maintainance;
    
    market.amm = Amm {
        oracle: ctx.accounts.oracle.key(),
        base_asset_reserve: params.base_asset_reserve,
        quote_asset_reserve: params.quote_asset_reserve,
        last_funding_rate: 0,
        last_funding_rate_ts: clock.unix_timestamp,
        amm_price: params.quote_asset_reserve / params.base_asset_reserve,
        k: params.base_asset_reserve * params.quote_asset_reserve,
        oracle_price_weight: 5000, // Default to 50% oracle weight
        last_oracle_update: clock.unix_timestamp,
    };
    
    market.bump = ctx.bumps.market;
    ctx.accounts.state.no_of_markets += 1;
    Ok(())
}

