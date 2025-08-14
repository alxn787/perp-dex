use anchor_lang::prelude::*;
use crate::utils::constraint::can_sign_for_user;
use crate::states::state::State;
use crate::states::user::User;
use crate::utils::error::Perperror;
use crate::states::user_map::UserMap;
use crate::states::perp_market_map::PerpMarketMap;
use crate::OrderStatus;

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

    fill_order(ctx, order_id, market_index)?;

    Ok(())
}

pub fn fill_order(ctx: Context<FillOrder>, order_id: u64, market_index: u16)->Result<()>{

    let perp_market_map = &ctx.remaining_accounts[0];

    let perp_market_map: PerpMarketMap = PerpMarketMap::try_from_slice(&perp_market_map.data.borrow())?;

    let user_map = &ctx.remaining_accounts[1];
    let user_map: UserMap = UserMap::try_from_slice(&user_map.data.borrow())?;

    fill_perp_order_controller(
        &ctx.accounts.state,
        &mut ctx.accounts.user,
        &mut ctx.accounts.filler,
        &perp_market_map,
        &user_map,
        order_id,
        market_index,
    )?;

    Ok(())
}

pub fn fill_perp_order_controller(
    state: &Account<State>,
    user: &mut Account<User>,
    filler: &mut Account<User>,
    perp_market_map: &PerpMarketMap,
    user_map: &UserMap,
    order_id: u64,
    market_index: u16,
)->Result<()>{

    let order_index = user
    .orders
    .iter()
    .position(|order| order.order_id == order_id && order.status == OrderStatus::Open)
    .ok_or(Perperror::OrderNotFound)?;

    let position_index = user
    .perp_positions // Fix: should be perp_positions, not positions
    .iter()
    .position(|position| position.market_index == market_index as u64) // Fix: cast to u64
    .ok_or(Perperror::UserHasNoPositionInMarket)?; // Fix: use correct error

    // Get immutable reference first
    let perp_market = perp_market_map.get_ref(market_index).ok_or(Perperror::InvalidMarketIndex)?;
    
    Ok(())
}