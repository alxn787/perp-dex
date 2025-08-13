use anchor_lang::prelude::*;
use crate::states::state::State;
use crate::states::user::User;

#[derive(Accounts)]
pub struct FillOrder<'info> {
    pub state: Account<'info, State>,

    pub authority: Signer<'info>,
    #[account(mut)]
    pub filler: Account<'info, User>,

    #[account(mut)]
    pub user: Account<'info, User>,

}
