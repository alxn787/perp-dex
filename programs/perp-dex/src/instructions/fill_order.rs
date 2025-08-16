use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use crate::utils::constraint::can_sign_for_user;
use crate::states::state::State;
use crate::states::user::User;
use crate::utils::error::Perperror;
use crate::states::user_map::UserMap;
use crate::states::perp_market_map::PerpMarketMap;
use crate::{fill_with_amm, fill_with_match, get_types_of_filling, FullfillmentMethod, Order, OrderStatus, OrderType, PositionDirection};

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
    let mut user_map: UserMap = UserMap::try_from_slice(&user_map.data.borrow())?;

    fill_perp_order_controller(
        &ctx.accounts.state,
        &mut ctx.accounts.user,
        &mut ctx.accounts.filler,
        &mut perp_market_map,
        &mut user_map,
        order_id,
        market_index,
    )?;

    Ok(())
}

pub fn fill_perp_order_controller(
    _state: &State,
    user: &mut User,
    _filler: &mut Account<User>,
    perp_market_map: &mut PerpMarketMap,
    maker_map: &mut UserMap,
    order_id: u64,
    market_index: u16,
)->Result<(u64,u64)>{

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
    .ok_or(Perperror::UserHasNoPositionInMarket)?; 

    let existing_base_asset_amount = user.perp_positions[position_index].base_asset_amount;


    let maker_id_index_price = get_maker_id_index_price(
        perp_market_map,
        &maker_map,
        &user_key,
        &user.orders[order_index],
    )?;

    let(base_asset_amount, quote_asset_amount) = execute_perp_order(
        user,
        order_index,
        maker_map,
        maker_id_index_price,
        perp_market_map,
    )?;
    
    Ok((base_asset_amount,quote_asset_amount))
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

        let mut _market = perp_market_map.get_mut(taker_order.market_index).ok_or(Perperror::InvalidMarketIndex)?;

        let idx_price_for_maker = get_idx_price_for_maker(
            &maker,
            &maker_direction,
            taker_order.market_index,
        )?;

        if idx_price_for_maker.is_empty() {
            continue;
        }

        for (idx, price) in idx_price_for_maker {

            let maker_order_price = price;
            let maker_order_idx = idx;

            let _maker_order = &maker.orders[maker_order_idx];

            let taker_order_price = taker_order.price.unwrap();

            if maker_order_price > taker_order_price {
                continue;
            }

            add_to_maker_order_info(
                &mut maker_idx_price,
                (*maker_key, maker_order_idx, maker_order_price),
                maker_direction,
            );
        }
        
        
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

        idx_price.push((order_index, order.price.unwrap()));
    }
    Ok(idx_price)
}

fn add_to_maker_order_info(
    maker_idx_price: &mut Vec<(Pubkey, usize, u64)>,
    current_maker_order_info: (Pubkey, usize, u64),
    direction: PositionDirection,
) {
    let price = current_maker_order_info.2;
    let index = match maker_idx_price.binary_search_by(|item| match direction {
        PositionDirection::Short => item.2.cmp(&price),
        PositionDirection::Long => price.cmp(&item.2),
    }) {
        Ok(index) => index,
        Err(index) => index,
    };

    if index < maker_idx_price.capacity() {
        maker_idx_price.insert(index, current_maker_order_info);
    }
}

pub fn execute_perp_order(
    taker: &mut User,
    taker_order_index: usize,
    maker_map: &mut UserMap,
    maker_id_index_price: Vec<(Pubkey, usize, u64)>,
    perp_market_map: &mut PerpMarketMap,
) -> Result<(u64,u64)> {
    let base_asset_amount = 0_u64;
    let quote_asset_amount = 0_u64;

    let market_index = taker.orders[taker_order_index].market_index;

    let market = perp_market_map.get_ref(market_index).ok_or(Perperror::InvalidMarketIndex)?;

    let limit_price = taker.orders[taker_order_index].price;

    let types_of_filling = get_types_of_filling(
        &taker.orders[taker_order_index],
        maker_id_index_price,
        &market.amm,
        limit_price,
    )?;

    if types_of_filling.is_empty(){
        return Ok((0,0));
    }
    let maker_direction = taker.orders[taker_order_index].opposite();
    let mut maker_fill_map : BTreeMap<Pubkey, i64> = BTreeMap::new();


    for type_of_filling in types_of_filling.iter() {
        let mut market = perp_market_map.get_mut(market_index).ok_or(Perperror::InvalidMarketIndex)?;
        let user_order_direction = taker.orders[taker_order_index].direction;

        let(filled_base_asset_amount, filled_quote_asset_amount) = match type_of_filling {

            FullfillmentMethod::AMM(Some(maker_price)) => {
                let(base_asset_filled, quote_asset_filled) = 
                    fill_with_amm(
                        taker,
                        taker_order_index,
                        maker_price.clone(),
                        market,
                    )?;

                (base_asset_filled, quote_asset_filled)
            }

            FullfillmentMethod::Match(maker_key, maker_order_idx, maker_price) => {

                let mut maker = maker_map.0.get_mut(&maker_key).ok_or(Perperror::InvalidMakerKey)?;

                let(base_asset_filled, quote_asset_filled) = fill_with_match(
                    taker,
                    taker_order_index,
                    maker,
                    *maker_order_idx as usize,
                    *maker_price,
                )?;

                (base_asset_filled, quote_asset_filled)
            }
            FullfillmentMethod::AMM(None) => {

                let(base_asset_filled, quote_asset_filled) = 
                    fill_with_amm(
                            taker,
                            taker_order_index,
                            limit_price.unwrap(),
                            market,
                    )?;   

            (base_asset_filled, quote_asset_filled)
            }
        };
    }

    
    Ok((base_asset_amount,quote_asset_amount))
}
