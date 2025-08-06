use anchor_lang::prelude::*;
use crate::states::*;
#[derive(Accounts)]
pub struct InitializePerpMarket<'info> {
    #[account(init,
        payer = authority,
        space = PerpMarket::LEN,
        seeds = [
            b"perp_market".as_ref(), 
              
        ],
        bump
    )]
    pub market: Account<'info, PerpMarket>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub state: Account<'info, State>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

