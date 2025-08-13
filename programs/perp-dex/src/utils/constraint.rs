use anchor_lang::prelude::*;
use crate::states::user::User;

pub fn can_sign_for_user(user: &Account<User>, signer: &Signer) -> anchor_lang::Result<bool> {
    Ok(user.authority.eq(signer.key))
}