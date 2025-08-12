use anchor_lang::prelude::*;

#[account]
pub struct State {
    pub admin: Pubkey,
    pub no_of_markets: u64,
    pub perp_fee: u64,
    pub no_of_users: u64,
    pub bump: u8,
    pub signer: Pubkey,
    pub signer_bump: u8,
}

impl State {
    pub const SIZE: usize = 8 + // discriminator
                           32 + // admin (Pubkey)
                           8 + // no_of_markets (u64)
                           8 + // perp_fee (u64)
                           8 + // no_of_users (u64)
                           1 + // bump (u8)
                           32 + // signer (Pubkey)
                           1;  // signer_bump (u8)
                           // Total: 98 bytes
}