use anchor_lang::prelude::*;
use crate::utils::constraint::can_sign_for_user;
use crate::states::state::State;
use crate::states::user::User;
use crate::Perperror;

#[derive(Accounts)]
pub struct FillOrder<'info> {
    pub state: Account<'info, State>,

    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = can_sign_for_user(&filler, &authority)?
    )]
    pub filler: Account<'info, User>,

    #[account(mut)]
    pub user: Account<'info, User>,

}

pub fn handle_fill_order(ctx: Context<FillOrder>, order_id: Option<u64>) -> Result<()> {

    let(order_id , market_index) = {
        let user = &ctx.accounts.user;
        let order_id = order_id.unwrap_or_else(|| user.get_last_order_id());
        let market_index = match user.get_order(order_id){
            Some(order)=> order.market_index,
            None => return Err(Perperror::OrderNotFound.into()),
        };
        (order_id, market_index)
    };

    Ok(())
}