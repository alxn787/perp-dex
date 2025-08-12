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
    #[msg("Insufficient Accounts")]
    InsufficientAccounts,
    #[msg("Max Number Of Positions")]
    MaxNumberOfPositions,
    #[msg("Max Number Of Orders")]
    MaxNumberOfOrders,
    #[msg("User Has No Position In Market")]
    UserHasNoPositionInMarket,
    #[msg("User Has No Order In Market")]
    UserHasNoOrderInMarket,
    #[msg("Invalid Leverage")]
    InvalidLeverage,
    #[msg("Invalid Price")]
    InvalidPrice,
}