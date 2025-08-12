use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use super::PerpMarket;

pub struct PerpMarketMap (pub BTreeMap<u16, PerpMarket>);

impl PerpMarketMap {
    pub fn get_ref(&self, market_index: u16) -> Option<&PerpMarket> {
        self.0.get(&market_index)
    }

    pub fn get_mut(&mut self, market_index: u16) -> Option<&mut PerpMarket> {
        self.0.get_mut(&market_index)
    }

    pub fn insert(&mut self, market_index: u16, market: PerpMarket) {
        self.0.insert(market_index, market);
    }

    pub fn remove(&mut self, market_index: u16) {
        self.0.remove(&market_index);
    }
}

impl AnchorSerialize for PerpMarketMap {
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

impl AnchorDeserialize for PerpMarketMap {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        // Deserialize the BTreeMap
        let len = u32::deserialize(buf)?;
        let mut map = BTreeMap::new();
        
        for _ in 0..len {
            let key = u16::deserialize(buf)?;
            let value = PerpMarket::deserialize(buf)?;
            map.insert(key, value);
        }
        
        Ok(PerpMarketMap(map))
    }

    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        // Deserialize the BTreeMap from reader
        let len = u32::deserialize_reader(reader)?;
        let mut map = BTreeMap::new();
        
        for _ in 0..len {
            let key = u16::deserialize_reader(reader)?;
            let value = PerpMarket::deserialize_reader(reader)?;
            map.insert(key, value);
        }
        
        Ok(PerpMarketMap(map))
    }
}

impl PerpMarketMap {
    pub const SIZE: usize = 8 + // discriminator
                           4 + // BTreeMap size (u32)
                           8 * (2 + PerpMarket::SIZE); // Assuming max 8 markets (u16 key + PerpMarket value)
                           // Total: 12 + 8*(2 + PerpMarket::SIZE) bytes
}