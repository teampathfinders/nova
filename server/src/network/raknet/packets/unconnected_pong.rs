use bytes::Bytes;
use bytes::{BufMut, BytesMut};

use crate::network::raknet::OFFLINE_MESSAGE_DATA;
use common::Serialize;
use common::VResult;
use common::WriteExtensions;

/// Response to [`OfflinePing`](super::offline_ping::OfflinePing).
#[derive(Debug)]
pub struct UnconnectedPong<'a> {
    /// Timestamp of when the ping was sent.
    /// This should be given the same value as [`OfflinePing::time`](super::offline_ping::OfflinePing::time).
    pub time: u64,
    /// Randomised GUID of the server.
    /// Corresponds to [`ServerInstance::guid`](crate::ServerInstance::guid)
    pub server_guid: u64,
    /// Contains the info to be displayed in the server banner in the server tab.
    /// Corresponds to [`ServerInstance::metadata`](crate::ServerInstance::metadata)
    pub metadata: &'a str,
}

impl UnconnectedPong<'_> {
    /// Unique identifier of this packet.
    pub const ID: u8 = 0x1c;

    pub fn serialized_size(&self) -> usize {
        1 + 8 + 8 + 16 + 2 + self.metadata.len()
    }
}

impl Serialize for UnconnectedPong<'_> {
    fn serialize(&self, buffer: &mut BytesMut) {
        buffer.put_u8(Self::ID);
        buffer.put_u64(self.time);
        buffer.put_u64(self.server_guid);
        buffer.put(OFFLINE_MESSAGE_DATA);
        buffer.put_raknet_string(self.metadata);
    }
}
