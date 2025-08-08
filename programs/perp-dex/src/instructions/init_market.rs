use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::states::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct InitializePerpMarket<'info> {
    
    #[account(init,
        payer = admin,
        space = PerpMarket::LEN,
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
            b"quote_asset_vault".as_ref(),state.no_of_markets.to_le_bytes().as_ref()  
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


pub fn handle_initialize_perp_market(ctx: Context<InitializePerpMarket>,market_index: u64) -> Result<()> {
    require!(market_index == ctx.accounts.state.no_of_markets, Perperror::InvalidMarketIndex);
    Ok(())
}
