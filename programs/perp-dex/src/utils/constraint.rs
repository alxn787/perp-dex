use anchor_lang::prelude::*;
use crate::{states::user::User};

pub fn can_sign_for_user(user: &Account<User>, signer: &Signer) -> anchor_lang::Result<bool> {
    Ok(user.authority.eq(signer.key))
}


#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Eq, Default)]
pub struct InitializeMarketParams {
    pub market_index: u64,
    pub base_asset_reserve: u64,
    pub quote_asset_reserve: u64,
    pub liquidator_fee: u64,
    pub max_leverage: u64,
    pub margin_ratio_initial: u64,
    pub margin_ratio_maintainance: u64,
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


#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum FullfillmentMethod {
    AMM(Option<u64>),
    Match(Pubkey, u16, u64),
}


#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Eq, Default)]
pub enum OrderType {
    Market,
    #[default]
    Limit,
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Eq, Default)]
pub enum OrderStatus {
    #[default]
    Open,
    Filled,
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Eq, Default)]
pub enum PositionDirection {
    #[default]
    Long,
    Short,
}
