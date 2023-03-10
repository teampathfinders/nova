use bytes::{BufMut, BytesMut, Bytes};
use common::{VResult, Vector, Vector3f, Vector3i, WriteExtensions, size_of_varint};

use common::Serialize;

use super::ConnectedPacket;

/// Plays a sound for the client.
#[derive(Debug)]
pub struct PlaySound<'s> {
    /// Name of the sound.
    pub name: &'s str,
    /// Position of the sound.
    pub position: Vector3i,
    /// Volume of the sound.
    pub volume: f32,
    /// Pitch of the sound.
    pub pitch: f32,
}

impl ConnectedPacket for PlaySound<'_> {
    const ID: u32 = 0x56;

    fn serialized_size(&self) -> usize {
        size_of_varint(self.name.len() as u32) + self.name.len() +
            3 * 4 + 4 + 4
    }
}

impl Serialize for PlaySound<'_> {
    fn serialize(&self, buffer: &mut BytesMut) {
        buffer.put_string(self.name);
        buffer.put_vec3i(&self.position);
        buffer.put_f32_le(self.volume);
        buffer.put_f32_le(self.pitch);
    }
}
