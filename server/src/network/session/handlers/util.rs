use bytes::BytesMut;
use common::{VResult, Vector3f, Vector3i, BlockPosition};

use crate::network::{
    packets::{
        AddPainting, ChangeDimension, CreditStatus, Difficulty, Dimension, GameMode, MessageType,
        MobEffectKind, MobEffectOperation, MobEffectUpdate, PaintingDirection, PlaySound,
        SetCommandsEnabled, SetDifficulty, SetPlayerGameMode, SetTime, SetTitle, ShowCredits,
        ShowProfile, TextMessage, TitleOperation, NetworkChunkPublisherUpdate,
    },
    session::Session,
    Decodable,
};

impl Session {
    pub fn handle_text_message(&self, packet: BytesMut) -> VResult<()> {
        let request = TextMessage::decode(packet)?;
        tracing::info!("{request:?}");

        let reply = SetTitle {
            remain_duration: 40,
            xuid: self.get_xuid()?.to_string(),
            operation: TitleOperation::SetTitle,
            text: format!("You said {}", request.message),
            platform_online_id: "".to_owned(),
            fade_in_duration: 10,
            fade_out_duration: 10,
        };
        self.send_packet(reply)?;

        let reply2 = NetworkChunkPublisherUpdate {
            position: BlockPosition::new(0, 0, 0),
            radius: 10
        };
        self.send_packet(reply2)?;

        Ok(())
    }
}