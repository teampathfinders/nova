use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use common::VResult;

use common::Serialize;

use super::ConnectedPacket;

const DEFINITIONS: &[u8] = include_bytes!("../../../included/biomes.nbt");

/// Sends a list of available biomes to the client.
#[derive(Debug, Clone)]
pub struct BiomeDefinitionList;

impl ConnectedPacket for BiomeDefinitionList {
    const ID: u32 = 0x7a;

    fn serialized_size(&self) -> usize {
        DEFINITIONS.len()
    }
}

impl Serialize for BiomeDefinitionList {
    fn serialize(&self, buffer: &mut BytesMut) {
        buffer.put(DEFINITIONS);
    }
}
