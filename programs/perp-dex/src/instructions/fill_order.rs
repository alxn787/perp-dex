use anchor_lang::prelude::*;
use crate::utils::constraint::can_sign_for_user;
use crate::states::state::State;
use crate::states::user::User;
use crate::utils::error::Perperror;
use crate::states::user_map::UserMap;
use crate::states::perp_market_map::PerpMarketMap;
use crate::{Order, OrderStatus, OrderType, PositionDirection};

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

    let mut perp_market_map: PerpMarketMap = PerpMarketMap::try_from_slice(&perp_market_map.data.borrow())?;

    let user_map = &ctx.remaining_accounts[1];
    let user_map: UserMap = UserMap::try_from_slice(&user_map.data.borrow())?;

    fill_perp_order_controller(
        &ctx.accounts.state,
        &mut ctx.accounts.user,
        &mut ctx.accounts.filler,
        &mut perp_market_map,
        &user_map,
        order_id,
        market_index,
    )?;

    Ok(())
}

pub fn fill_perp_order_controller(
    state: &State,
    user: &User,
    filler: &mut Account<User>,
    perp_market_map: &mut PerpMarketMap,
    maker_map: &UserMap,
    order_id: u64,
    market_index: u16,
)->Result<()>{

    let user_key = user.authority;

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

    let maker_id_index_price = get_maker_id_index_price(
        perp_market_map,
        &maker_map,
        &user_key,
        &user.orders[order_index],
    )?;
    
    Ok(())
}

pub fn  get_maker_id_index_price(
    perp_market_map: &mut PerpMarketMap,
    maker_map: &UserMap,
    taker_key: &Pubkey,
    taker_order: &Order
)->Result<Vec<(Pubkey, usize,u64)>>{
    let mut maker_idx_price = Vec::with_capacity(8);

    let maker_direction = taker_order.opposite();

    for(maker_key, maker) in maker_map.0.iter() {

        if maker_key == taker_key {
            continue;
        }

        let _maker = maker;

        let mut market = perp_market_map.get_mut(taker_order.market_index).ok_or(Perperror::InvalidMarketIndex)?;

        let idx_price_for_maker = get_idx_price_for_maker(
            &maker,
            &maker_direction,
            taker_order.market_index,
        )?;

        
    }

    Ok(maker_idx_price)
}

fn get_idx_price_for_maker(
    maker: &User,
    maker_direction: &PositionDirection,
    market_index: u16,
)->Result<Vec<(usize, u64)>>{

    let mut idx_price = Vec::with_capacity(8);

    for(order_index, order) in maker.orders.iter().enumerate() {
        if order.status != OrderStatus::Open || order.market_index != market_index || order.direction != *maker_direction || order.order_type != OrderType::Limit {
            continue;
        }

        idx_price.push((order_index, order.price));
    }


    Ok(idx_price)
}