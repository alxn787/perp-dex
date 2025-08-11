use anchor_lang::prelude::*;
use crate::states::*;
use crate::error::Perperror;

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
}


pub fn handle_place_order(ctx: Context<PlaceOrder>, params: OrderParams) -> Result<()> {

    require!(ctx.remaining_accounts.len() > 0, Perperror::InsufficientAccounts);
    let perp_market_map = &ctx.remaining_accounts[0];

    let perp_market_map: PerpMarketMap = PerpMarketMap::try_from_slice(&perp_market_map.data.borrow())?;

    let state = &ctx.accounts.state;
    let mut user = &mut ctx.accounts.user;

    place_order(ctx, params, &perp_market_map, &state, &mut user)?;

    Ok(())
}

pub fn place_order(
    ctx: Context<PlaceOrder>,
    params: OrderParams,
    perp_market_map: &PerpMarketMap,
    state: &State, 
    user: &mut User) -> Result<()> {

    let market = perp_market_map.get_ref(params.market_index).ok_or(Perperror::InvalidMarketIndex)?;

    let user_order_id = user.next_order_id;

    let order = Order{
        status: OrderStatus::Open,
        market_index: params.market_index,
        order_index: user_order_id,
        base_asset_amount: params.base_asset_amount,
        base_asset_amount_filled: 0,
        quote_asset_amount_filled: 0,
        price: params.price,
        direction: params.direction,
        order_type: params.order_type,
        leverage: 1,
    };


    Ok(())
}