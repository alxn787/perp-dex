use anchor_lang::prelude::*;

#[error_code]
pub enum Perperror {
    #[msg("Invalid Market Index")]
    InvalidMarketIndex,
    #[msg("Invalid Token Account")]
    InvalidTokenAccount,
    #[msg("Invalid Mint")]
    InvalidMint,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Invalid Oracle")]
    InvalidOracle,
    #[msg("Invalid Drift")]
    InvalidDrift,
    #[msg("Invalid AMM")]
    InvalidAmm,
    #[msg("Invalid Liquidator")]
    InvalidLiquidator,
}