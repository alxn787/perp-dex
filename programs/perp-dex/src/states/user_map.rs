use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use crate::states::user::User;

pub struct UserMap (pub BTreeMap<Pubkey, User>);

impl UserMap {
    pub fn get_ref(&self, user_key: &Pubkey) -> Option<&User> {
        self.0.get(user_key)
    }

    pub fn get_mut(&mut self, user_key: &Pubkey) -> Option<&mut User> {
        self.0.get_mut(user_key)
    }

    pub fn insert(&mut self, user_key: Pubkey, user: User) {
        self.0.insert(user_key, user);
    }

    pub fn remove(&mut self, user_key: &Pubkey) {
        self.0.remove(user_key);
    }
}

impl AnchorSerialize for UserMap {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        // Serialize the BTreeMap
        let map = &self.0;
        let len = map.len() as u32;
        len.serialize(writer)?;
        
        for (key, value) in map {
            key.serialize(writer)?;
            value.serialize(writer)?;
        }
        Ok(())
    }
}

impl AnchorDeserialize for UserMap {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        // Deserialize the BTreeMap
        let len = u32::deserialize(buf)?;
        let mut map = BTreeMap::new();
        
        for _ in 0..len {
            let key = Pubkey::deserialize(buf)?;
            let value = User::deserialize(buf)?;
            map.insert(key, value);
        }
        
        Ok(UserMap(map))
    }

    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        // Deserialize the BTreeMap from reader
        let len = u32::deserialize_reader(reader)?;
        let mut map = BTreeMap::new();
        
        for _ in 0..len {
            let key = Pubkey::deserialize_reader(reader)?;
            let value = User::deserialize_reader(reader)?;
            map.insert(key, value);
        }
        
        Ok(UserMap(map))
    }
}

impl UserMap {
    pub const SIZE: usize = 8 + // discriminator
                           4 + // BTreeMap size (u32)
                           8 * (32 + User::SIZE); // Assuming max 8 users (Pubkey key + User value)
                           // Total: 12 + 8*(32 + User::SIZE) bytes
}