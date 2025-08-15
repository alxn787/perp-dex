use crate::states::order::Order;
use crate::states::amm::Amm;
use crate::utils::constraint::PerpFulfillmentMethod;
use crate::utils::constraint::PositionDirection;
use anchor_lang::prelude::*;

pub fn get_types_of_filling(
    order: &Order,
    maker_id_index_price: Vec<(Pubkey, usize, u64)>,
    amm: &Amm,
    limit_price: Option<u64>,
)->Result<Vec<PerpFulfillmentMethod>>{

    let mut types_of_filling = Vec::with_capacity(8);

    let maker_direction = order.opposite();

    let amm_price = match maker_direction {
        PositionDirection::Long => amm.get_bid_price(),
        PositionDirection::Short => amm.get_ask_price(),
    };

    

    Ok(types_of_filling)
}   