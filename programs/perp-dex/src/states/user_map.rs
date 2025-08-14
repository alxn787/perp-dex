use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use crate::states::user::User;

pub struct UserMap(pub BTreeMap<Pubkey, Account<User>>);

impl UserMap<'a> {
    pub fn get_ref(&self, user_key: &Pubkey) -> Option<&Account<User>> {
        self.0.get(user_key)
    }

    pub fn get_mut(&mut self, user_key: &Pubkey) -> Option<&mut Account<User>> {
        self.0.get_mut(user_key)
    }

    pub fn insert(&mut self, user_key: Pubkey, user: Account<User>) {
        self.0.insert(user_key, user);
    }

    pub fn remove(&mut self, user_key: &Pubkey) {
        self.0.remove(user_key);
    }

}