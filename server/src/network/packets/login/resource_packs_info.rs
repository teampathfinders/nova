use bytes::Bytes;
use bytes::{BufMut, BytesMut};

use crate::network::packets::ConnectedPacket;
use common::{Serialize, size_of_varint, VarString};
use common::VResult;
use common::WriteExtensions;

/// Behavior pack information.
#[derive(Debug, Clone)]
pub struct BehaviorPack {
    /// UUID of the behavior pack.
    /// Each behavior pack must have a unique UUID.
    pub uuid: String,
    /// Version of the behavior pack.
    /// This allows the client to cache behavior packs.
    pub version: String,
    /// Size of the compressed archive of the behavior pack in bytes.
    pub size: u64,
    /// Key used to decrypt the packet if it is encrypted.
    /// This is generally used for marketplace packs.
    pub content_key: String,
    /// Subpack name.
    pub subpack_name: String,
    /// Another UUID required for marketplace and encrypted behavior packs.
    pub content_identity: String,
    /// Whether the pack contains script.
    /// If it does, the pack will only be downloaded if the client supports scripting.
    pub has_scripts: bool,
}

impl BehaviorPack {
    fn serialized_size(&self) -> usize {
        8 + 1 +
        self.uuid.var_len() +
        self.version.var_len() +
        self.content_key.var_len() +
        self.subpack_name.var_len() +
        self.content_identity.var_len()
    }
}

/// Resource pack information
#[derive(Debug, Clone)]
pub struct ResourcePack {
    /// UUID of the resource pack.
    /// Each resource pack must have a unique UUID.
    pub uuid: String,
    /// Version of the resource pack.
    /// This allows the client to cache resource packs.
    pub version: String,
    /// Size of the compressed archive of the resource pack in bytes.
    pub size: u64,
    /// Key used to decrypt the pack if it is encrypted.
    /// This is generally used for marketplace packs.
    pub content_key: String,
    /// Subpack name.
    pub subpack_name: String,
    /// Another UUID required for marketplace and encrypted resource packs.
    pub content_identity: String,
    /// Whether the pack contains scripts.
    /// If it does, the pack will only be downloaded if the client supports scripting.
    pub has_scripts: bool,
    /// Whether the pack uses raytracing.
    pub rtx_enabled: bool,
}

impl ResourcePack {
    fn serialized_size(&self) -> usize {
        8 + 1 + 1 +
        self.uuid.var_len() +
        self.version.var_len() +
        self.content_key.var_len() +
        self.subpack_name.var_len() +
        self.content_identity.var_len()
    }
}

/// Contains information about the addons used by the server.
/// This should be sent after sending the [`PlayStatus`](super::PlayStatus) packet with a
/// [`LoginSuccess`](super::Status::LoginSuccess) status.
/// 
/// If the server has no resource packs, a [`ResourcePackStack`](super::ResourcePackStack) packet can be sent immediately after this one
/// to prevent a client response.
#[derive(Debug)]
pub struct ResourcePacksInfo<'a> {
    /// Forces the client to accept the packs to be able to join the server.
    pub required: bool,
    /// Indicates whether there are packs that make use of scripting.
    pub scripting_enabled: bool,
    /// Unknown what this does.
    pub forcing_server_packs: bool,
    /// List of behavior packs
    pub behavior_info: &'a [BehaviorPack],
    /// List of resource packs.
    pub resource_info: &'a [ResourcePack],
}

impl ConnectedPacket for ResourcePacksInfo<'_> {
    const ID: u32 = 0x06;

    fn serialized_size(&self) -> usize {
        1 + 1 + 1 + 2 + 2 +
        self.behavior_info.iter().fold(
            0, |acc, p| acc + p.serialized_size()
        ) +
        self.resource_info.iter().fold(
            0, |acc, p| acc + p.serialized_size()
        )
    }
}

impl Serialize for ResourcePacksInfo<'_> {
    fn serialize(&self, buffer: &mut BytesMut) {
        buffer.put_bool(self.required);
        buffer.put_bool(self.scripting_enabled);
        buffer.put_bool(self.forcing_server_packs);

        buffer.put_u16(self.behavior_info.len() as u16);
        for pack in self.behavior_info {
            buffer.put_string(&pack.uuid);
            buffer.put_string(&pack.version);
            buffer.put_u64(pack.size);
            buffer.put_string(&pack.content_key);
            buffer.put_string(&pack.subpack_name);
            buffer.put_string(&pack.content_identity);
            buffer.put_bool(pack.has_scripts);
        }

        buffer.put_u16(self.resource_info.len() as u16);
        for pack in self.resource_info {
            buffer.put_string(&pack.uuid);
            buffer.put_string(&pack.version);
            buffer.put_u64(pack.size);
            buffer.put_string(&pack.content_key);
            buffer.put_string(&pack.subpack_name);
            buffer.put_string(&pack.content_identity);
            buffer.put_bool(pack.has_scripts);
            buffer.put_bool(pack.rtx_enabled);
        }
    }
}
