use bytes::{BytesMut, Bytes};
use common::{bail, ReadExtensions, VError, VResult, WriteExtensions, size_of_varint};

use common::{Deserialize, Serialize};

use crate::network::packets::ConnectedPacket;

/// The Minecraft game modes.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
    /// Sets the player's game mode to the world default.
    WorldDefault = 5,
}

impl TryFrom<i32> for GameMode {
    type Error = VError;

    fn try_from(value: i32) -> VResult<Self> {
        Ok(match value {
            0 => Self::Survival,
            1 => Self::Creative,
            2 => Self::Adventure,
            3 => Self::Spectator,
            5 => Self::WorldDefault,
            _ => bail!(BadPacket, "Invalid game mode"),
        })
    }
}

/// Sets the player's game mode.
#[derive(Debug, Clone)]
pub struct SetPlayerGameMode {
    /// Game mode to apply.
    pub game_mode: GameMode,
}

impl ConnectedPacket for SetPlayerGameMode {
    const ID: u32 = 0x3e;

    fn serialized_size(&self) -> usize {
        size_of_varint(self.game_mode as i32)
    }
}

impl Serialize for SetPlayerGameMode {
    fn serialize(&self, buffer: &mut BytesMut) {
        buffer.put_var_i32(self.game_mode as i32);
    }
}

impl Deserialize for SetPlayerGameMode {
    fn deserialize(mut buffer: Bytes) -> VResult<Self> {
        let game_mode = GameMode::try_from(buffer.get_var_i32()?)?;
        Ok(Self { game_mode })
    }
}
