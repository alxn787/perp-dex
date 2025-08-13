use anchor_lang::prelude::*;
use crate::utils::constraint::can_sign_for_user;
use crate::states::perp_market_map::PerpMarketMap;
use crate::states::position::{add_new_position, get_position_index, update_bids_and_asks};
use crate::utils::error::Perperror;
use crate::utils::constant::*;
use crate::states::order::{Order, OrderType, PositionDirection, OrderStatus};
use crate::states::state::State;
use crate::states::user::User;

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    pub state: Account<'info, State>,
    #[account(
        mut,
        constraint = can_sign_for_user(&user, &authority)?
    )]
    pub user: Account<'info, User>,
    pub authority: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
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
    _state: &State, 
    user: &mut User) -> Result<()> {

    // Validate input parameters
    require!(params.base_asset_amount >= MIN_ORDER_AMOUNT, Perperror::InvalidAmount);
    require!(params.leverage >= MIN_LEVERAGE && params.leverage <= MAX_LEVERAGE, Perperror::InvalidLeverage);
    require!(params.price > 0, Perperror::InvalidPrice);
    require!(params.market_index <= MAX_MARKET_INDEX, Perperror::InvalidMarketIndex);

    let _market = perp_market_map.get_ref(params.market_index).ok_or(Perperror::InvalidMarketIndex)?;

    
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
        order_id: user_order_id,
        status: OrderStatus::Open,
    };

    user.orders[new_order_index] = order;
    user.perp_positions[position_index].open_orders += 1;
    user.next_order_id += 1;
    user.open_orders += 1;
    update_bids_and_asks(&mut user.perp_positions[position_index], params.direction, params.base_asset_amount)?;

    Ok(())
}