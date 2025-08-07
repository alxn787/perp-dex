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

pub fn handle_initialize(ctx: Context<Initialize>, perp_fee: u64) -> Result<()> {

    let (drift_signer, bump) = Pubkey::find_program_address(&[b"drift_signer".as_ref()], ctx.program_id);

    let state = &mut ctx.accounts.state;
    state.admin = ctx.accounts.admin.key();
    state.no_of_markets = 0;
    state.perp_fee = perp_fee;
    state.bump = bump;
    state.signer = drift_signer;
    Ok(())
}