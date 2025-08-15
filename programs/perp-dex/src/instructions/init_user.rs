use anchor_lang::prelude::*;
use crate::states::user::User;
use crate::states::state::State;


#[derive(Accounts)]
#[instruction(
    account_id: u16,
)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        seeds = [b"user", payer.key.as_ref(), account_id.to_le_bytes().as_ref()],
        space = User::SIZE,
        bump,
        payer = payer
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub state: Account<'info, State>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_user(ctx:Context<InitializeUser>, account_id: u16) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let state = &mut ctx.accounts.state;

    user.next_order_id = 1;
    user.authority = ctx.accounts.payer.key();
    user.open_orders = 0;
    user.total_collateral = 0;
    user.account_id = account_id;

    state.no_of_users += 1;
    Ok(())
}