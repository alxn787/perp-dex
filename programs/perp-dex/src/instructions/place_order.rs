use anchor_lang::prelude::*;
use crate::states::perp_market_map::PerpMarketMap;
use crate::states::position::{add_new_position, get_position_index};
use crate::utils::error::Perperror;
use crate::states::order::{Order, OrderType, PositionDirection, OrderStatus};
use crate::states::state::State;
use crate::states::user::User;

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    pub state: Account<'info, State>,
    #[account(
        mut,
    )]
    pub user: Account<'info, User>,
    pub authority: Signer<'info>,
}

pub struct OrderParams {
    pub order_type: OrderType,
    pub direction: PositionDirection,
    pub user_order_id: u8,
    pub base_asset_amount: u64,
    pub price: u64,
    pub market_index: u16, 
    pub leverage: u64,
}

pub fn handle_place_order(ctx: Context<PlaceOrder>, params: OrderParams) -> Result<()> {

    let perp_market_map = &ctx.remaining_accounts[0];

    let perp_market_map: PerpMarketMap = PerpMarketMap::try_from_slice(&perp_market_map.data.borrow())?;

    let state = &ctx.accounts.state.clone();
    let mut user = &mut ctx.accounts.user;

    place_order(params, &perp_market_map, &state, &mut user)?;

    Ok(())
}

pub fn place_order(
    params: OrderParams,
    perp_market_map: &PerpMarketMap,
    state: &State, 
    user: &mut User) -> Result<()> {

    let market = perp_market_map.get_ref(params.market_index).ok_or(Perperror::InvalidMarketIndex)?;

    
    let new_order_index = user
    .orders
    .iter()
    .position(|order| order.is_available())
    .ok_or(Perperror::MaxNumberOfOrders)?;

    let user_order_id = user.next_order_id;

    let position_index = get_position_index(&user.perp_positions, params.market_index)
        .or_else(|_| add_new_position(&mut user.perp_positions, params.market_index))?;
   
    let order = Order{
        market_index: params.market_index,
        order_index: new_order_index as u64,
        base_asset_amount: params.base_asset_amount,
        base_asset_amount_filled: 0,
        quote_asset_amount_filled: 0,
        price: params.price,
        direction: params.direction,
        order_type: params.order_type,
        leverage: params.leverage,
        status: OrderStatus::Open,
    };


    user.orders[new_order_index] = order;
    user.next_order_id += 1;
    user.open_orders += 1;

    Ok(())
}