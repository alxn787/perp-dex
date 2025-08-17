use anchor_lang::prelude::*;
use anchor_spl::{token::{self, Transfer}, token_interface::{TokenAccount, TokenInterface}};
use crate::{get_position_index, utils::Perperror, State, User};

#[derive(Accounts)]
#[instruction(market_index: u16,)]
pub struct Withdraw<'info> {
    pub state: Account<'info, State>,
    #[account(
        mut,
        has_one = authority,
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
        constraint = state.signer.eq(&drift_signer.key())
    )]
    /// CHECK: forced drift_signer
    pub drift_signer: AccountInfo<'info>,
    #[account(
        mut,
        constraint = &perp_market_vault.mint.eq(&user_token_account.mint)
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handle_withdraw(ctx: Context<Withdraw>, market_index: u16, amount: u64) -> Result<()> {
    let perp_market_vault = &mut ctx.accounts.perp_market_vault;
    let user_token_account = &mut ctx.accounts.user_token_account;
    let state = &mut ctx.accounts.state;

    let cpi_accounts = Transfer {
        from: perp_market_vault.to_account_info(),
        to: user_token_account.to_account_info(),
        authority: ctx.accounts.drift_signer.to_account_info(),
    };

    let seeds = [b"drift_signer".as_ref(),&[state.signer_bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_context, amount)?;

    let user = &mut ctx.accounts.user;
    user.total_collateral = user.total_collateral.checked_sub(amount).ok_or(Perperror::ArithmeticOverflow)?;
    let position_index = get_position_index(&mut user.perp_positions, market_index)?;
    user.perp_positions[position_index].collateral = user.perp_positions[position_index].collateral.checked_sub(amount).ok_or(Perperror::ArithmeticOverflow)?;
    Ok(())
}
