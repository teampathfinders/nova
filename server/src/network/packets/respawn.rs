use bytes::{Buf, BufMut, BytesMut, Bytes};
use common::{
    bail, ReadExtensions, VError, VResult, Vector3f, WriteExtensions, size_of_varint,
};

use common::{Deserialize, Serialize};

use super::ConnectedPacket;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RespawnState {
    Searching,
    ServerReady,
    ClientReady,
}

impl TryFrom<u8> for RespawnState {
    type Error = VError;

    fn try_from(value: u8) -> VResult<Self> {
        Ok(match value {
            0 => Self::Searching,
            1 => Self::ServerReady,
            2 => Self::ClientReady,
            _ => bail!(BadPacket, "Invalid respawn state {value}"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Respawn {
    pub position: Vector3f,
    pub state: RespawnState,
    pub runtime_id: u64,
}

impl ConnectedPacket for Respawn {
    const ID: u32 = 0x2d;

    fn serialized_size(&self) -> usize {
        3 * 4 + 1 + size_of_varint(self.runtime_id)
    }
}

impl Serialize for Respawn {
    fn serialize(&self, buffer: &mut BytesMut) {
        buffer.put_vec3f(&self.position);
        buffer.put_u8(self.state as u8);
        buffer.put_var_u64(self.runtime_id);
    }
}

impl Deserialize for Respawn {
    fn deserialize(mut buffer: Bytes) -> VResult<Self> {
        let position = buffer.get_vec3f();
        let state = RespawnState::try_from(buffer.get_u8())?;
        let runtime_id = buffer.get_var_u64()?;

        Ok(Self { position, state, runtime_id })
    }
}
