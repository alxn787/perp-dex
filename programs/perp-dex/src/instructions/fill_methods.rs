use std::cmp::min;

use crate::get_position_index;
use crate::states::order::Order;
use crate::states::amm::Amm;
use crate::utils::constraint::FullfillmentMethod;
use crate::utils::constraint::PositionDirection;
use crate::states::user::User;
use crate::states::market::PerpMarket;
use crate::utils::Perperror;
use crate::OrderStatus;
use anchor_lang::prelude::*;

pub fn get_types_of_filling(
    order: &Order,
    maker_id_index_price: Vec<(Pubkey, usize, u64)>,
    amm: &Amm,
    limit_price: Option<u64>,
)->Result<Vec<FullfillmentMethod>>{

    let mut types_of_filling = Vec::with_capacity(8);

    let maker_direction = order.opposite();

    let mut amm_price = match maker_direction {
        PositionDirection::Long => amm.get_bid_price(),
        PositionDirection::Short => amm.get_ask_price(),
    };

    for (maker_key, maker_order_idx, maker_order_price) in maker_id_index_price {

        let taker_cross_maker = match limit_price {
            Some(limit_price) => does_order_cross(
                &maker_direction,
                maker_order_price,
                limit_price,
            ),
            None => true,
        };

        // break intead of continue because array is sorted . later orders will be worse .. no point in going through it 
        if !taker_cross_maker {
            continue;
        }

        let is_maker_better_than_amm = match maker_direction {
            PositionDirection::Long => maker_order_price < amm_price,
            PositionDirection::Short => maker_order_price > amm_price,
        };

        if !is_maker_better_than_amm {
            types_of_filling.push(FullfillmentMethod::AMM(Some(maker_order_price)));
            amm_price = maker_order_price;
        }

        // taker crosses maker , maker is better than amm , add maker order
        types_of_filling.push(FullfillmentMethod::Match(maker_key, maker_order_idx as u16, maker_order_price));

        if types_of_filling.len() >= 6 {
            break;
        }
    }

    // at last fill the remaining with amm
    let taker_crosses_amm = match limit_price {
        Some(taker_price) => does_order_cross(&maker_direction, amm_price, taker_price),
        None => true,
    };

    if taker_crosses_amm {
        types_of_filling.push(FullfillmentMethod::AMM(None));
    }
    
    Ok(types_of_filling)
}   

pub fn does_order_cross(
    maker_direction: &PositionDirection,
    maker_order_price: u64,
    limit_price: u64,
)->bool{
    match maker_direction {
        PositionDirection::Long => limit_price > maker_order_price,
        PositionDirection::Short => limit_price < maker_order_price,
    }
}

pub fn fill_with_amm(
    user: &mut User,
    order_index: usize,
    _limit_price: Option<u64>,
    market: &mut PerpMarket,
)->Result<(u64, u64)>{

   
    let existing_base_asset_amount =  user.orders[order_index].base_asset_amount;

    let quote_amount = match _limit_price {
        Some(limit_price) => market.amm.calculate_quote_for_base_with_limit(existing_base_asset_amount , limit_price)?,
        None => market.amm.calculate_quote_for_base_no_limit(existing_base_asset_amount)?,
    };

    if quote_amount == 0 {
        return Ok((0,0));
    }

    market.amm.execute_trade(existing_base_asset_amount, quote_amount)?;

    update_order_after_filling(
        &mut user.orders[order_index],
        existing_base_asset_amount,
        quote_amount
    )?;


    Ok((existing_base_asset_amount, quote_amount))
}

pub fn fill_with_match(
    taker: &mut User,
    taker_order_index: usize,
    taker_limit_price: Option<u64>,
    maker: &mut User,
    maker_order_index: usize,
    maker_price: u64,
)->Result<(u64, u64)>{

    let maker_direction = maker.orders[maker_order_index].direction;

    let taker_limit_price = match taker_limit_price {
        Some(limit_price) => limit_price,
        None => maker_price,
    };

    require!(taker.orders[taker_order_index].opposite() == maker.orders[maker_order_index].direction, Perperror::InvalidDirection);


    let taker_base_asset_amount = taker.orders[taker_order_index]
    .get_unfilled_base()?;

    let maker_base_asset_amount = maker.orders[maker_order_index].get_unfilled_base()?;

    let does_order_cross = does_order_cross(&maker_direction, maker_price, taker_limit_price);

    if !does_order_cross {
        return Ok((0,0));
    }

    let (filled_base_asset_amount, filled_quote_asset_amount) = calculate_fill_by_match(
        maker_base_asset_amount,
        maker_price,
        taker_base_asset_amount,    
    )?;

    update_order_after_filling(
        &mut maker.orders[maker_order_index],
        filled_base_asset_amount,
        filled_quote_asset_amount
    )?;

    update_order_after_filling(
        &mut taker.orders[taker_order_index],
        filled_base_asset_amount,
        filled_quote_asset_amount
    )?;


    Ok((0,0))
}

pub fn update_order_after_filling(
    order: &mut Order,
    base_asset_amount: u64,
    quote_asset_amount: u64,
) -> Result<()> {
    order.base_asset_amount_filled = order.base_asset_amount_filled.checked_add(base_asset_amount)
    .ok_or(Perperror::ArithmeticOverflow)?;

    order.quote_asset_amount_filled = order
        .quote_asset_amount_filled
        .checked_add(quote_asset_amount)
        .ok_or(Perperror::ArithmeticOverflow)?;

    if order.get_unfilled_base()? == 0 {
        order.status = OrderStatus::Filled;
    }

    Ok(())
}

pub fn calculate_fill_by_match(
    maker_base_asset_amount: u64,
    maker_price: u64,
    taker_base_asset_amount: u64,
)->Result<(u64, u64)>{
    let filled_base_asset_amount = min(maker_base_asset_amount, taker_base_asset_amount);
    let filled_quote_asset_amount = filled_base_asset_amount * maker_price;

    Ok((filled_base_asset_amount, filled_quote_asset_amount))
}
