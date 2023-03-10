use bytes::Bytes;
use bytes::{BufMut, BytesMut};

use crate::network::raknet::OFFLINE_MESSAGE_DATA;
use common::Serialize;
use common::VResult;

/// Sent in response to [`OpenConnectionRequest1`](super::open_connection_request1::OpenConnectionRequest1).
#[derive(Debug)]
pub struct OpenConnectionReply1 {
    /// GUID of the server.
    /// Corresponds to [`ServerInstance::guid`](crate::ServerInstance::guid).
    pub server_guid: u64,
    /// MTU of the connection.
    /// This should be given the same value as [`OpenConnectionRequest1::mtu`](super::open_connection_request1::OpenConnectionRequest1::mtu).
    pub mtu: u16,
}

impl OpenConnectionReply1 {
    /// Unique identifier of this packet.
    pub const ID: u8 = 0x06;

    pub fn serialized_size(&self) -> usize {
        1 + 16 + 8 + 1 + 2
    }
}

impl Serialize for OpenConnectionReply1 {
    fn serialize(&self, buffer: &mut BytesMut) {
        buffer.put_u8(Self::ID);
        buffer.put(OFFLINE_MESSAGE_DATA);
        buffer.put_u64(self.server_guid);
        // Disable security, required for login sequence.
        // Encryption will be enabled later on.
        buffer.put_u8(0);
        buffer.put_u16(self.mtu);
    }
}
