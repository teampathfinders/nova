use bytes::{BufMut, BytesMut, Bytes};
use common::{Serialize, VResult, WriteExtensions};
use crate::network::packets::ConnectedPacket;

#[derive(Debug, Clone)]
pub struct ContainerClose {
    /// Equal to the window ID sent in the [`ContainerOpen`](super::ContainerOpen) packet.
    pub window_id: u8,
    /// Whether the server force-closed the container.
    pub server_initiated: bool
}

impl ConnectedPacket for ContainerClose {
    const ID: u32 = 0x2f;

    fn serialized_size(&self) -> usize {
        2
    }
}

impl Serialize for ContainerClose {
    fn serialize(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.window_id);
        buffer.put_bool(self.server_initiated);
    }
}