use bytes::{BytesMut, Bytes};
use common::{VResult, WriteExtensions};

use common::Serialize;

use crate::network::packets::ConnectedPacket;

/// Enables or disables the usage of commands.
///
/// If commands are disabled, the client will prevent itself from even sending any.
#[derive(Debug, Clone)]
pub struct SetCommandsEnabled {
    /// Whether commands are enabled.
    pub enabled: bool,
}

impl ConnectedPacket for SetCommandsEnabled {
    const ID: u32 = 0x3b;

    fn serialized_size(&self) -> usize {
        1
    }
}

impl Serialize for SetCommandsEnabled {
    fn serialize(&self, buffer: &mut BytesMut) {
        buffer.put_bool(self.enabled);
    }
}
