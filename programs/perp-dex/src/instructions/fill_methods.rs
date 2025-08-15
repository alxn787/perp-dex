use crate::states::order::Order;
use crate::states::amm::Amm;
use crate::utils::constraint::FullfillmentMethod;
use crate::utils::constraint::PositionDirection;
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