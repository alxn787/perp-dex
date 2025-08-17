use anchor_lang::prelude::*;
use anchor_spl::{token::{self, Transfer}, token_interface::{TokenAccount, TokenInterface}};
use crate::{can_sign_for_user, State, User};
use crate::utils::error::Perperror;
use crate::states::position::get_forced_position_from_market_index;

#[derive(Accounts)]
#[instruction(market_index: u16)]
pub struct Deposit<'info> {
    pub state: Account<'info, State>,
    #[account(
        mut,
        constraint = can_sign_for_user(&user, &authority)?
    )]
    pub user: Account<'info, User>,

    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"perp_market_vault".as_ref(), market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub perp_market_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = &perp_market_vault.mint.eq(&user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handle_deposit(ctx: Context<Deposit>, market_index: u16, amount: u64) -> Result<()> {
    let perp_market_vault = &mut ctx.accounts.perp_market_vault;
    let user_token_account = &mut ctx.accounts.user_token_account;

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: user_token_account.to_account_info(),
            to: perp_market_vault.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        }
    );
    token::transfer(cpi_context, amount)?;

    let user = &mut ctx.accounts.user;
    user.total_collateral = user.total_collateral.checked_add(amount).ok_or(Perperror::ArithmeticOverflow)?;
    let position_index = get_forced_position_from_market_index(&mut user.perp_positions, market_index)?;
    user.perp_positions[position_index].collateral = user.perp_positions[position_index].collateral.checked_add(amount).ok_or(Perperror::ArithmeticOverflow)?;
    Ok(())
}