use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, Token}};
use crate::states::*;

#[derive(Accounts)]
pub struct  Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        seeds = [b"drift_state".as_ref()],
        space = State::SIZE,
        bump,
        payer = admin
    )]
    pub state: Box<Account<'info, State>>,
    pub quote_asset_mint: Account<'info, Mint>,
    /// CHECK: checked in `initialize`
    pub drift_signer: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}